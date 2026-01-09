
use windows::{
    core::PWSTR,
    Win32::UI::Shell::{SHGetKnownFolderPath, KNOWN_FOLDER_ID, KF_FLAG_DEFAULT},
};
use winreg::RegKey;
use std::path::{Path, PathBuf};
use glob::glob;

#[derive(serde::Serialize)]
pub struct InstalledApp {
    pub name: String,
    pub path: Option<String>,  // None 表示是 Store 应用或找不到 exe
}

pub fn get_installed_apps() -> Vec<InstalledApp> {
    let mut apps = Vec::new();

    // 1. 读取开始菜单快捷方式（覆盖率最高，最快）
    apps.extend(get_apps_from_start_menu());

    // 2. 补充从注册表读取传统卸载项（补全没创建快捷方式的程序）
    apps.extend(get_apps_from_uninstall_registry());

    // 3. 去重（按路径去重，保留名字最“干净”的那个）
    dedup_apps(&mut apps);

    apps
}

// ──────────────────────────────────────────────────────────────
// 1. 从开始菜单读取 .lnk
fn get_apps_from_start_menu() -> Vec<InstalledApp> {
    let mut results = Vec::new();

    let paths = vec![
        // 当前用户
        get_start_menu_path(false),
        // 所有用户
        get_start_menu_path(true),
    ];

    for base in paths.into_iter().flatten() {
        let pattern = base.join("**").join("*.lnk");
        if let Ok(entries) = glob(&pattern.to_string_lossy()) {
            for entry in entries.flatten() {
                if let Ok(target) = resolve_lnk(&entry) {
                    if target.exists() && target.extension().map(|s| s == "exe").unwrap_or(false) {
                        let name = entry.file_name()
                            .unwrap_or_default()
                            .to_string_lossy()
                            .replace(".lnk", "");
                        results.push(InstalledApp {
                            name: name.trim().to_string(),
                            path: Some(target.to_string_lossy().to_string()),
                        });
                    }
                }
            }
        }
    }
    results
}

// 获取开始菜单路径
fn get_start_menu_path(all_users: bool) -> Option<PathBuf> {
    let folder_id = if all_users {
        // FOLDERID_CommonStartMenu
        KNOWN_FOLDER_ID(0xA4115719u32 as _, 0xD62Eu16, 0x491Du16, [0xA8, 0x94, 0xC6, 0x57, 0x5F, 0xB8, 0xB9, 0xB2])
    } else {
        // FOLDERID_StartMenu  
        KNOWN_FOLDER_ID(0x625Bu16, 0x3E0Bu16, 0x4B0Au16, [0x90, 0xB8, 0xB8, 0xB8, 0xB8, 0xB8, 0xB8, 0xB8])
    };

    unsafe {
        let mut path = PWSTR::null();
        if SHGetKnownFolderPath(&folder_id, KF_FLAG_DEFAULT, None, &mut path).is_ok() {
            let s = path.to_string().ok()?;
            CoTaskMemFree(Some(path.0 as _));
            Some(PathBuf::from(s).join("Programs"))
        } else {
            None
        }
    }
}

// 解析 .lnk 快捷方式
fn resolve_lnk(path: &Path) -> Result<PathBuf, ()> {
    use windows::Win32::UI::Shell::IShellLinkW;
    use windows::Win32::Foundation::S_OK;
    use std::ptr;

    let link: IShellLinkW = unsafe { windows::Win32::System::Com::CoCreateInstance(&windows::Win32::UI::Shell::ShellLink, None, windows::Win32::System::Com::CLSCTX_INPROC_SERVER) }.map_err(|_| ())?;

    let persist: windows::Win32::System::Com::IPersistFile = link.cast().map_err(|_| ())?;
    unsafe { persist.Load(&windows::core::HSTRING::from(path.to_string_lossy().as_ref()), 0) }.ok().map_err(|_| ())?;

    let mut resolved = [0u16; 260];
    unsafe { link.Resolve(tauri::api::process::current_process().main_window().unwrap().hwnd().unwrap() as _, 0) }.ok();
    unsafe { link.GetPath(resolved.as_mut_ptr(), resolved.len() as _, ptr::null_mut(), 0x1) }
        .ok().map_err(|_| ())?;

    let path_str = String::from_utf16_lossy(&resolved);
    let path_str = path_str.trim_end_matches('\0');
    Ok(PathBuf::from(path_str))
}

// ──────────────────────────────────────────────────────────────
// 2. 从注册表 Uninstall 键读取（补全没快捷方式的）
fn get_apps_from_uninstall_registry() -> Vec<InstalledApp> {
    let mut results = Vec::new();
    let hklm = RegKey::predef(winreg::enums::HKEY_LOCAL_MACHINE);
    let hkcu = RegKey::predef(winreg::enums::HKEY_CURRENT_USER);

    for root in &[hklm, hkcu] {
        let uninstall = match root.open_subkey(r"SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall") {
            Ok(k) => k,
            _ => continue,
        };

        for key_name in uninstall.enum_keys().flatten() {
            if let Ok(app) = uninstall.open_subkey(&key_name) {
                if let (Ok(name), Ok(path)) = (app.get_value::<String, _>("DisplayName"), app.get_value::<String, _>("InstallLocation")) {
                    let exe = find_exe_in_dir(&path).or_else(|| {
                        app.get_value::<String, _>("UninstallString").ok()
                            .and_then(|s| s.split('\"').nth(1)?.to_string())
                            .filter(|p| p.ends_with(".exe"))
                    });

                    if name.trim() != "" {
                        results.push(InstalledApp {
                            name: name.trim().to_string(),
                            path: exe,
                        });
                    }
                }
            }
        }
    }
    results
}

fn find_exe_in_dir(dir: &str) -> Option<String> {
    if dir.is_empty() { return None; }
    let path = Path::new(dir);
    if !path.exists() { return None; }

    // 优先找常见入口
    for name in &["*.exe", "bin\\*.exe", "app\\*.exe"] {
        if let Ok(mut entries) = glob(&path.join(name).to_string_lossy()) {
            if let Some(Ok(p)) = entries.next() {
                return Some(p.to_string_lossy().to_string());
            }
        }
    }
    None
}

// ──────────────────────────────────────────────────────────────
// 去重（同路径保留名字最短的）
fn dedup_apps(apps: &mut Vec<InstalledApp>) {
    apps.sort_by_key(|a| a.path.clone().unwrap_or_default());
    apps.dedup_by_key(|a| a.path.clone().unwrap_or_default());
}

// 在 main.rs 注册命令
fn main() {
    get_installed_apps();
}