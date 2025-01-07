use windows::{
    core::*, Win32::Foundation::*, Win32::System::Threading::*, Win32::System::ProcessStatus::*,
    Win32::System::LibraryLoader::*, Win32::UI::WindowsAndMessaging::*,
};

fn main() -> Result<()> {
    unsafe {
        let cmd: Vec<u16> = "c:\\system32\\cmd.exe".encode_utf16().chain(Some(0)).collect();
        let mut args: Vec<u16> = "/C echo 111".encode_utf16().chain(Some(0)).collect();
        let si = STARTUPINFOW {
            cb: std::mem::size_of::<STARTUPINFOW>() as u32,
            dwFlags: STARTF_USESHOWWINDOW,
            wShowWindow: 0,
            ..Default::default()
        };
        let mut pi = PROCESS_INFORMATION::default();

        let res = CreateProcessW(
            PCWSTR(cmd.as_ptr()),
            PWSTR(args.as_mut_ptr()),
            None,
            None,
            BOOL(0), // 不继承句柄
            CREATE_NO_WINDOW, // 隐藏窗口
            None,
            None,
            &si,
            &mut pi,
        );

        if res.is_ok() {
            // 等待进程完成
            WaitForSingleObject(pi.hProcess, INFINITE);
            // 关闭句柄
            CloseHandle(pi.hProcess);
            CloseHandle(pi.hThread);
        } else {
            eprintln!("无法创建进程: {:?}", res);
        }
    }
    Ok(())
}