mod bindings;

use libloading::Library;
use bindings::LibraryBindings;

fn main() -> Result<(), String> {
    unsafe {
        let lib_name = if cfg!(target_os = "linux") {
            "NativeCSharp.so"
        } else if cfg!(target_os = "windows") {
            "NativeCSharp.dll"
        } else {
            return Err("Unsupported operating system!".to_string());
        };

        // Load the shared library
        let lib = Library::new(lib_name)
            .map_err(|e| e.to_string())?;
        let bindings = LibraryBindings::new(&lib)?;

        // Use binding struct to call functions
        let int_value = bindings.get_int();
        println!("[RUST]: Integer value from native .NET: '{}'.", int_value);

        let string_value = bindings.get_string()?;
        println!("[RUST]: String value (now owned by rust code) from native .NET: '{}'.", string_value);

        // Calling shutdown is needed otherwise the .NET runtime will cause a segmentation fault when dropped because it's being interrupted.
        // You can test this behaviour by commenting the following line out
        bindings.shutdown();
        println!("[RUST]: The native .NET library is now going to be dropped.");
    }

    println!("[RUST]: The native .NET library has successfully been dropped.");
    Ok(())
}
