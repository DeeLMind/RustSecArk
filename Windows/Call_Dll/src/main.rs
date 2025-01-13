use windows::core::PCWSTR;
use windows::Win32::System::LibraryLoader::LoadLibraryW;

fn main() {
    // Load the DLL
    let dll_path = PCWSTR("create_dll.dll".encode_utf16().collect::<Vec<u16>>().as_ptr());

    let hmodule = match unsafe { LoadLibraryW(dll_path) } {
        Ok(h) => h,
        Err(e) => {
            eprintln!("Failed to load DLL: {}", e);
            return;
        }
    };
}