use libloading::{Library, Symbol};
use std::ffi::CStr;
use std::os::raw::c_char;

fn main() -> Result<(), String> {
    unsafe {
        // Load the shared library
        let lib = Library::new("../NativeCSharp/bin/Release/linux-x64/publish/NativeCSharp.so")
            .map_err(|e| e.to_string())?;

        // Load and call the GetInt function
        let get_int: Symbol<unsafe extern "C" fn() -> i32> = lib.get(b"GetInt").map_err(|e| e.to_string())?;
        let result = get_int();
        println!("Result from C# GetInt: {}", result);

        // Load and call the GetString function
        let get_string: Symbol<unsafe extern "C" fn() -> *const c_char> = lib.get(b"GetString").map_err(|e| e.to_string())?;
        let c_string_ptr = get_string();

        if c_string_ptr.is_null() {
            return Err("Received a null pointer from C# GetString".into());
        }

        // Convert the C string to a Rust String
        let c_str = CStr::from_ptr(c_string_ptr);
        let rust_string = c_str.to_string_lossy().into_owned();

        println!("Received string from C#: {}", rust_string);

        let free_string: Symbol<unsafe extern "C" fn(*const c_char)> = lib.get(b"FreeString").map_err(|e| e.to_string())?;
        free_string(c_string_ptr);

        // gracefully shutting down the library which includes the .net runtime running in the background
        // not doing this results in a segfault when rust drops the library.
        let shutdown: Symbol<unsafe extern "C" fn()> = lib.get(b"Shutdown").map_err(|e| e.to_string())?;
        shutdown();

        println!("Leaving unsafe block!");
    }

    println!("Left unsafe block!");
    Ok(())
}
