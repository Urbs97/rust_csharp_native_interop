use libloading::{Library, Symbol};

fn main() -> Result<(), String> {
    unsafe {
        let lib = Library::new("../NativeCSharp/bin/Release/linux-x64/publish/NativeCSharp.so").map_err(|e| e.to_string())?;
        let func: Symbol<unsafe extern "C" fn() -> i32> = lib.get(b"GetInt").map_err(|e| e.to_string())?;
        let result = func();
        println!("Result from C#: {}", result);
    }

    Ok(())
}
