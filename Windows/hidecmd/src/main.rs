use windows::{
    core::*, Win32::Foundation::*, Win32::System::Threading::*, Win32::System::ProcessStatus::*,
    Win32::System::LibraryLoader::*, Win32::UI::WindowsAndMessaging::*,
};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 假设你要运行 "notepad.exe"
    let mut cmd = Command::new("cmd.exe")
        .arg("/c")
        .arg("notepad.exe")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;

    unsafe {
        let mut startup_info: STARTUPINFOW = std::mem::zeroed();
        startup_info.cb = std::mem::size_of::<STARTUPINFOW>() as DWORD;
        GetStartupInfoW(&mut startup_info as *mut _ as LPVOID);

        if !startup_info.hStdInput.is_null() {
            ShowWindow(startup_info.hStdInput, SW_HIDE);
        }
    }

    // 等待进程结束（可选）
    let _ = cmd.wait()?;

    Ok(())
}