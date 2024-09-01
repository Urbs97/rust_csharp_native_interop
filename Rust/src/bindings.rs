use libloading::{Library, Symbol};
use std::ffi::CStr;
use std::os::raw::c_char;

pub struct LibraryBindings<'a> {
    get_int: Symbol<'a, unsafe extern "C" fn() -> i32>,
    get_string: Symbol<'a, unsafe extern "C" fn() -> *const c_char>,
    free_string: Symbol<'a, unsafe extern "C" fn(*const c_char)>,
    shutdown: Symbol<'a, unsafe extern "C" fn()>,
}

impl<'a> LibraryBindings<'a> {
    pub unsafe fn new(lib: &'a Library) -> Result<Self, String> {
        let get_int = lib.get(b"GetInt").map_err(|e| e.to_string())?;
        let get_string = lib.get(b"GetString").map_err(|e| e.to_string())?;
        let free_string = lib.get(b"FreeString").map_err(|e| e.to_string())?;
        let shutdown = lib.get(b"Shutdown").map_err(|e| e.to_string())?;

        Ok(LibraryBindings {
            get_int,
            get_string,
            free_string,
            shutdown,
        })
    }

    pub fn get_int(&self) -> i32 {
        unsafe { (self.get_int)() }
    }

    pub fn get_string(&self) -> Result<String, String> {
        unsafe {
            let c_string_ptr = (self.get_string)();
            if c_string_ptr.is_null() {
                return Err("Received a null pointer from GetString".into());
            }
            let c_str = CStr::from_ptr(c_string_ptr);
            let rust_string = c_str.to_string_lossy().into_owned();
            (self.free_string)(c_string_ptr);
            Ok(rust_string)
        }
    }

    pub fn shutdown(&self) {
        unsafe { (self.shutdown)() }
    }
}
