use windows::Win32::Foundation::{BOOL, HINSTANCE, HWND};
use windows::core::w;
use windows::Win32::System::SystemServices::DLL_PROCESS_ATTACH;
use windows::Win32::UI::WindowsAndMessaging::MessageBoxW;

#[no_mangle]
#[allow(non_snake_case, unused_variables)]
extern "system" fn DllMain(hinstance: HINSTANCE, reason: u32, _: *mut std::ffi::c_void) -> BOOL {
    match reason {
        DLL_PROCESS_ATTACH => {
            unsafe {
                MessageBoxW(Some(HWND(std::ptr::null_mut())), w!("Hello"), w!("dll"), Default::default());
            }
        },
        _ => {}
    }
    true.into()
}
