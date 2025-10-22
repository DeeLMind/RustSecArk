use libloading::Library;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::path::PathBuf;

type BOOL = i32;
type DWORD = u32;
type WCHAR = u16;

fn to_wide_null(s: &str) -> Vec<WCHAR> {
    OsStr::new(s).encode_wide().chain(Some(0)).collect()
}

fn from_wide_null(buf: &[WCHAR]) -> String {
    let len = buf.iter().position(|&c| c == 0).unwrap_or(buf.len());
    String::from_utf16_lossy(&buf[..len])
}

pub struct Everything {
    lib: Library,
}

impl Everything {
    pub fn load(path: Option<&str>) -> Result<Self, libloading::Error> {
        if let Some(p) = path {
            let lib = unsafe { Library::new(p)? };
            return Ok(Self { lib });
        }
        // fallback attempts
        let lib_res = unsafe {
            Library::new("Everything64.dll").or_else(|_| Library::new("Everything.dll"))
        };
        match lib_res {
            Ok(lib) => Ok(Self { lib }),
            Err(e) => Err(e),
        }
    }

    pub fn set_search(&self, q: &str) -> Result<bool, Box<dyn std::error::Error>> {
        unsafe {
            let f: libloading::Symbol<unsafe extern "system" fn(*const WCHAR) -> BOOL> =
                self.lib.get(b"Everything_SetSearchW\0")?;
            let w = to_wide_null(q);
            let r = f(w.as_ptr());
            Ok(r != 0)
        }
    }

    pub fn query(&self, wait: bool) -> Result<bool, Box<dyn std::error::Error>> {
        unsafe {
            let f: libloading::Symbol<unsafe extern "system" fn(BOOL) -> BOOL> =
                self.lib.get(b"Everything_QueryW\0")?;
            let r = f(if wait { 1 } else { 0 });
            Ok(r != 0)
        }
    }

    pub fn get_num_results(&self) -> Result<u32, Box<dyn std::error::Error>> {
        unsafe {
            let f: libloading::Symbol<unsafe extern "system" fn() -> DWORD> =
                self.lib.get(b"Everything_GetNumResults\0")?;
            Ok(f())
        }
    }

    pub fn get_result_fullpath(
        &self,
        index: u32,
    ) -> Result<Option<String>, Box<dyn std::error::Error>> {
        unsafe {
            let f: libloading::Symbol<
                unsafe extern "system" fn(DWORD, *mut WCHAR, DWORD) -> DWORD,
            > = self.lib.get(b"Everything_GetResultFullPathNameW\0")?;

            // initial buffer
            let mut buf: Vec<WCHAR> = vec![0; 1024];
            let needed = f(index as DWORD, buf.as_mut_ptr(), buf.len() as DWORD);
            if needed == 0 {
                return Ok(None);
            }
            if (needed as usize) >= buf.len() {
                // reallocate
                let mut buf2: Vec<WCHAR> = vec![0; (needed as usize) + 1];
                let needed2 = f(index as DWORD, buf2.as_mut_ptr(), buf2.len() as DWORD);
                if needed2 == 0 {
                    return Ok(None);
                }
                return Ok(Some(from_wide_null(&buf2)));
            }
            Ok(Some(from_wide_null(&buf)))
        }
    }

    pub fn search(&self, q: &str, wait: bool) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
        self.set_search(q)?;
        self.query(wait)?;
        let n = self.get_num_results()?;
        let mut out = Vec::with_capacity(n as usize);
        for i in 0..n {
            if let Some(s) = self.get_result_fullpath(i)? {
                out.push(PathBuf::from(s));
            }
        }
        Ok(out)
    }
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let everything = Everything::load(None)?;
    let query = "test";
    let results = everything.search(query, true)?;
    println!("Found {} results", results.len());
    for p in results.iter().take(300) {
        println!("{}", p.display());
    }
    Ok(())
}