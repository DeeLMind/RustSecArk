static WINGET: &str = "D:\\UniGetUI\\winget-cli_x64\\winget.exe";

use serde::{Deserialize, Serialize};
use tokio::process::Command; // 使用 tokio 的 Command
use tokio::time::{timeout, Duration};
use std::process::{ Stdio};
use serde_json::Value;
use std::{
    ffi::OsStr
};

/// Windows ONLY：CREATE_NO_WINDOW
#[cfg(windows)]
const CREATE_NO_WINDOW: u32 = 0x0800_0000;
//
/// 执行winget命令行，返回搜索结果字符串
///
/// # 参数
///
/// * `exe_path` - 可执行文件路径
/// * `args` - 参数迭代器
/// * `hide_window` - 是否在 Windows 下隐藏黑框
/// * `wait` - 超时时间
///
/// # 返回值
///
/// * `Result<String, String>` - 返回搜索结果字符串，或错误信息
pub async fn exec_cmd<S, A>(
    exe_path: S,              // 可执行文件
    args: A,                  // 参数迭代器
    hide_window: bool,        // Windows 下是否隐藏黑框
    wait: Duration,           // 超时时间
) -> Result<String, String>
where
    S: AsRef<OsStr>,
    A: IntoIterator,
    A::Item: AsRef<OsStr>,
{
    // -------- 组装命令 --------
    let mut cmd = Command::new(exe_path);
    cmd.args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .stdin(Stdio::null());

    // Windows：隐藏窗口
    #[cfg(windows)]
    if hide_window {
        cmd.creation_flags(CREATE_NO_WINDOW);
    }

    // -------- 运行并限时 --------
    let run = cmd.output();
    let result = timeout(wait, run).await;

    // -------- 统一结果处理 --------
    match result {
        // 子进程正常结束
        Ok(Ok(output)) => {
            if !output.stdout.is_empty() {
                String::from_utf8(output.stdout)
                    .map_err(|e| format!("无法解析 stdout: {e}"))
            } else if !output.stderr.is_empty() {
                // winget 常把错误写 stderr
                String::from_utf8(output.stderr)
                    .map(|e| Err(e)).unwrap_or_else(|e| Err(format!("无法解析 stderr: {e}")))
            } else {
                Err("命令无任何输出".into())
            }
        }
        // 子进程启动失败
        Ok(Err(e)) => Err(format!("命令启动失败: {e}")),
        // 超时
        Err(_) => Err("命令执行超时".into()),
    }
}
//
// fn get_chars_slice(s: &str, start: usize, end: usize) -> String {
//     s.chars().skip(start).take(end - start).collect()
// }
//
// fn get_chars_from(s: &str, start: usize) -> String {
//     s.chars().skip(start).collect()
// }
//
// #[derive(Serialize, Debug)]
// struct Software {
//     name: String,
//     id: String,
//     version: String,
//     source: String,
// }
//
// fn process_packages(reader: &str) -> Result<String, String> {
//     let mut reader = reader.to_string();
//     let mut packages = Vec::new();
//     let mut old_line = String::new();
//     let mut id_index = -1;
//     let mut version_index = -1;
//     let mut source_index = -1;
//     let mut dashes_passed = false;
//
//     if let Some(pos) = reader.find("Name") {
//         reader = reader[pos..].to_string();
//     }
//
//     for line in reader.lines() {
//         if !dashes_passed && line.contains("---") {
//             let header_prefix = if old_line.contains("SearchId") { "Search" } else { "" };
//
//             id_index = old_line
//                 .find(&(header_prefix.to_string() + "Id"))
//                 .unwrap_or(usize::MAX) as i32;
//             version_index = old_line
//                 .find(&(header_prefix.to_string() + "Version"))
//                 .unwrap_or(usize::MAX) as i32;
//             source_index = old_line
//                 .find(&(header_prefix.to_string() + "Source"))
//                 .unwrap_or(usize::MAX) as i32;
//
//             dashes_passed = true;
//         } else if dashes_passed
//             && id_index > 0
//             && version_index > 0
//             && id_index < version_index
//             && (version_index as usize) < line.len()
//         {
//             let mut offset = 0;
//             while line.chars().nth((id_index - offset - 1) as usize) != Some(' ')
//                 && offset <= (id_index - 5)
//             {
//                 offset += 1;
//             }
//
//             // loop {
//             //     let check_pos = id_index - offset - 1;
//             //
//             //     // 终止条件1：下标越界
//             //     if check_pos < 0 {
//             //         println!("终止：check_pos < 0，offset={}", offset);
//             //         break;
//             //     }
//             //
//             //     // 取出字符
//             //     let current_char = line.chars().nth(check_pos as usize).unwrap_or(' ');
//             //     println!(
//             //         "检查位置 {} 的字符为 '{}', offset={}",
//             //         check_pos, current_char, offset
//             //     );
//             //
//             //     // 终止条件2：找到空格并且 offset 不大于 id_index - 5
//             //     let stop = current_char == ' ' && offset <= (id_index - 5);
//             //     if stop {
//             //         println!("终止：找到空格且 offset 合理，offset={}", offset);
//             //         break;
//             //     }
//             //
//             //     // 继续向左推进 offset
//             //     offset += 1;
//             //
//             //     // 防止无限循环：加个安全退出阈值（可选）
//             //     if offset > id_index {
//             //         println!("警告：offset 超过 id_index（{}），强制退出循环", id_index);
//             //         break;
//             //     }
//             // }
//
//             let name = {
//                 let name_part = get_chars_slice(line, 0, (id_index - offset) as usize);
//                 name_part.trim().to_string()
//             };
//
//             let id = {
//                 let id_part = get_chars_from(line, (id_index - offset) as usize);
//                 id_part.trim().split_whitespace().next().unwrap_or("").to_string()
//             };
//
//             let version = {
//                 let version_part = get_chars_from(line, (version_index - offset) as usize);
//                 version_part.trim().split_whitespace().next().unwrap_or("").to_string()
//             };
//
//             let source = {
//                 let source_start = (source_index - offset) as usize;
//                 if source_index == -1 || source_start >= line.chars().count() {
//                     "winget".to_string()
//                 } else {
//                     let source_part = get_chars_from(line, source_start);
//                     source_part.trim().split_whitespace().next().unwrap_or("winget").to_string()
//                 }
//             };
//
//             packages.push(Software {
//                 name,
//                 id,
//                 version,
//                 source,
//             });
//         }
//
//         old_line = line.to_string();
//     }
//
//     serde_json::to_string(&packages).map_err(|e| e.to_string())
// }
//
//
// /// 搜索软件
// ///
// /// # 参数
// ///
// /// * `winget_path` - winget可执行文件路径
// /// * `software_name` - 软件名称
// /// * `source` - 软件源，是软件筛选winget、msstore等
// ///
// /// # 返回值
// ///
// /// * `Result<String, String>` - 返回搜索结果字符串，或错误信息
// pub async fn search_winget(winget_path: &str,software_name: &str) -> Result<String, String> {
//     let result =exec_cmd(
//         winget_path,
//         vec!["search", software_name],
//         true,
//         std::time::Duration::from_secs(15),
//     ).await?;
//     process_packages(&result)
// }
//
// #[derive(Debug, Deserialize)]
// struct AppInfo {
//     name: String,
//     id: String,
//     version: String,
//     #[serde(rename = "source")]
//     source: Option<String>, // 有些项目可能字段写错了
// }
//
// fn print_apps(value: &Value) {
//     // 尝试将 Value 转为 Vec<AppInfo>
//     match serde_json::from_value::<Vec<AppInfo>>(value.clone()) {
//         Ok(apps) => {
//             for app in apps {
//                 println!(
//                     "名称: {:<20} | ID: {:<30} | 版本: {:<12} | 来源: {}",
//                     app.name,
//                     app.id,
//                     app.version,
//                     app.source.unwrap_or_else(|| "未知".to_string())
//                 );
//             }
//         }
//         Err(e) => {
//             eprintln!("解析 Value 失败: {}", e);
//         }
//     }
// }
//
// #[tokio::main]
// async fn main() {
//     let result = search_winget(WINGET,"wechat").await.unwrap();
//     println!("{}",result);
//     let parsed:Value = serde_json::from_str(result.as_str()).unwrap();
//     print_apps(&parsed);
// }

#[derive(serde::Serialize)]
pub struct WingetList {
    name: String,
    id: String,
    version: String,
}

// 工具函数：按字符位置截取字符串（start..end）
fn get_chars_range(s: &str, start: usize, end: usize) -> String {
    s.chars().skip(start).take(end - start).collect()
}

// 工具函数：从字符位置 start 开始一直取到结尾
fn get_chars_from(s: &str, start: usize) -> String {
    s.chars().skip(start).collect()
}

pub fn parse_list_info(result: &str) -> Result<String, String> {
    let mut reader = result.to_string();
    let mut packages = Vec::new();
    let mut old_line = String::new();
    let mut id_index = -1;
    let mut version_index = -1;
    let mut dashes_passed = false;

    if let Some(pos) = reader.find("Name") {
        reader = reader[pos..].to_string();
    }

    for line in reader.lines() {
        if !dashes_passed && line.contains("---") {
            let header_prefix = if old_line.contains("SearchId") {
                "Search"
            } else {
                ""
            };

            id_index = old_line
                .find(&(header_prefix.to_string() + "Id"))
                .unwrap_or(usize::MAX) as i32;

            version_index = old_line
                .find(&(header_prefix.to_string() + "Version"))
                .unwrap_or(usize::MAX) as i32;

            dashes_passed = true;
        } else if dashes_passed
            && id_index > 0
            && version_index > 0
            && id_index < version_index
            && (version_index as usize) < line.len()
        {
            let mut offset = 0;
            while (id_index - offset - 1) >= 0
                && (line
                .chars()
                .nth((id_index - offset - 1) as usize)
                .unwrap_or(' ')
                != ' '
                || offset > (id_index - 5))
            {
                offset += 1;
            }


            let name = {
                let end = (id_index - offset) as usize;
                let part = get_chars_range(line, 0, end);
                part.trim().to_string()
            };

            let id = {
                let start = (id_index - offset) as usize;
                let part = get_chars_from(line, start);
                part.trim().split_whitespace().next().unwrap_or("").to_string()
            };

            let version = {
                let start = (version_index - offset) as usize;
                let part = get_chars_from(line, start);
                part.trim().split_whitespace().next().unwrap_or("").to_string()
            };

            packages.push(WingetList {
                name,
                id,
                version,
            });
        }

        old_line = line.to_string();
    }

    serde_json::to_string(&packages).map_err(|e| e.to_string())
}


/// 列出本地软件并返回 JSON 格式
pub async fn list(winget_path:&str) -> Result<String, String> {
    println!("Listing installed software using winget at: {}", winget_path);
    let result = exec_cmd(
        winget_path,
        vec!["list","-s","winget"],
        true,
        std::time::Duration::from_secs(15),
    ).await?;
    println!("[winget] list result: {}", result);
    let app_info = parse_list_info(&result)?;
    Ok(app_info)
}

#[tokio::main]
async fn main() {
    list(WINGET).await;
}