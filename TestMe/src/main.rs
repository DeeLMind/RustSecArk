use std::ffi::OsStr;
use std::time::Duration;
use std::process::Stdio;
use tokio::process::Command;
use tokio::time::timeout;
use encoding_rs::GBK;

/// Windows ONLY：CREATE_NO_WINDOW
#[cfg(windows)]
const CREATE_NO_WINDOW: u32 = 0x0800_0000;

/// 执行winget命令行，返回搜索结果字符串
///
/// # 参数
///
/// * exe_path - 可执行文件路径
/// * args - 参数迭代器
/// * hide_window - 是否在 Windows 下隐藏黑框
/// * wait - 超时时间
///
/// # 返回值
///
/// * Result<String, String> - 返回搜索结果字符串，或错误信息
pub async fn exec_cmd<S, A>(
    exe_path: S,       // 可执行文件
    args: A,           // 参数迭代器
    hide_window: bool, // Windows 下是否隐藏黑框
    wait: Duration,    // 超时时间
) -> Result<String, String>
where
    S: AsRef<OsStr>,
    A: IntoIterator,
    A::Item: AsRef<OsStr>,
{
    // 组装命令
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

    // 运行并限时
    let run = cmd.output();
    let result = timeout(wait, run).await;

    // 统一结果处理
    match result {
        Ok(Ok(output)) => {
            let raw = if !output.stdout.is_empty() {
                output.stdout
            } else {
                output.stderr
            };

            // 尝试按 UTF-8 解析
            match String::from_utf8(raw.clone()) {
                Ok(s) => Ok(s),
                Err(_) => {
                    // 回退到 GBK 解码（Windows 常用编码）
                    let (cow, _, _) = GBK.decode(&raw);
                    Ok(cow.into_owned())
                }
            }
        }
        Ok(Err(e)) => Err(format!("命令启动失败: {e}")),
        Err(_) => Err("命令执行超时".into()),
    }
} 

pub async fn get_start_apps() -> Result<String,String> {
    let result = exec_cmd(
        "powershell",
        vec!["-Command", "Get-StartApps"],
        true,
        std::time::Duration::from_secs(20)
        ).await?;
    print!("XXX{}", result);
    Ok("List of start apps".to_string())
}

fn main() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        match get_start_apps().await {
            Ok(apps) => println!("Start Apps:\n{}", apps),
            Err(e) => eprintln!("Error: {}", e),
        }
    });
}