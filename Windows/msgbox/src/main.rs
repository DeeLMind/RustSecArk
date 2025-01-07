use windows::{
    core::*, Win32::Foundation::*, Win32::UI::WindowsAndMessaging::*,
};

fn main() -> Result<()> {
    unsafe {
        // 使用 encode_utf16 转换字符串到 UTF-16
        let message: Vec<u16> = "这是一个消息框的示例。".encode_utf16().chain(Some(0)).collect();
        let title: Vec<u16> = "标题".encode_utf16().chain(Some(0)).collect();

        MessageBoxW(
            None,
            PCWSTR(message.as_ptr()),
            PCWSTR(title.as_ptr()),
            MB_OK | MB_ICONINFORMATION
        );
    }
    Ok(())
}