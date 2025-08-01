use windows::{
    core::*,
    Win32::Foundation::*,
    Win32::System::LibraryLoader::GetModuleHandleW,
    Win32::UI::WindowsAndMessaging::*,
};

static mut HOOK_HANDLE: Option<HHOOK> = None;

unsafe extern "system" fn keyboard_proc(n_code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    unsafe {
        if n_code == HC_ACTION as i32 {
            let kb_struct = *(l_param.0 as *const KBDLLHOOKSTRUCT);
            if w_param.0 == WM_KEYDOWN as usize {
                let vk_code = kb_struct.vkCode;
                println!("Key pressed: {}",  vk_code as u8 as char);
            }
        }
        CallNextHookEx(HOOK_HANDLE, n_code, w_param, l_param)
    }
}

fn main() -> Result<()> {
    unsafe {
        let h_instance = HINSTANCE(GetModuleHandleW(None)?.0);

        let hook = SetWindowsHookExW(
            WH_KEYBOARD_LL,
            Some(keyboard_proc),
            Some(h_instance),
            0,
        )?;

        HOOK_HANDLE = Some(hook);

        let mut msg = MSG::default();

        while GetMessageW(&mut msg, None, 0, 0).into() {
            let _ = TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }

        if let Some(hook) = HOOK_HANDLE {
            let _ = UnhookWindowsHookEx(hook);
        }
    }

    Ok(())
}