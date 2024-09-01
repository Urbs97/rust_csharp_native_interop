# Rust + C# Native Interop

This repository explores the possibility of calling C# code from Rust when the C# code is compiled into a native library using .NET Native AOT. The project serves as a proof of concept for integrating Rust with C#, leveraging native interop for high-performance applications.

## Project Overview

The primary goal of this project is to demonstrate how Rust can call functions from a C# library that has been compiled into a native shared library using .NET Native AOT. This setup is particularly useful for scenarios where Rust needs to interact with C# logic while maintaining the performance benefits of native code execution.

### Current Status

- **C# to Rust Interop:** Successfully implemented. Rust can now call functions from a C# library compiled with .NET Native AOT.
- **Rust to C# Interop:** To be implemented. The next step is to enable calling Rust functions from C#.

## Project Structure

- **`NativeCSharp/`**: This directory contains the C# project. The C# code is compiled into a native shared library (`.so` for Linux, `.dll` for Windows) using .NET Native AOT.
- **`Rust/`**: This directory contains the Rust project, which dynamically loads the C# native library and calls its functions.

## Getting Started

### Prerequisites

- **Rust**: Ensure that Rust is installed on your system. You can install it via [rustup](https://rustup.rs/).
- **.NET SDK**: Install the .NET SDK for building the C# project. The SDK can be downloaded from [Microsoft's official website](https://dotnet.microsoft.com/download).
- **Platform-Specific Tools**: Depending on your platform, ensure you have the necessary tools for compiling native code:
  - On **Windows**: Visual Studio (Installer) with the C++ build tools.
  - On **Linux**: GCC or Clang for compiling native code.

### Building the Project

1. **Build the C# Native Library**:
   - Navigate to the `NativeCSharp` directory.
   - Build the C# project using the appropriate configuration (`Debug` or `Release`). For example:
     ```bash
     dotnet publish -c Release -r linux-x64 --self-contained
     ```
     or
     ```bash
     dotnet publish -c Release -r win-x64 --self-contained
     ```
     > Note that the rust build script always expects an **release** build.
   - The output will be a native shared library located in `NativeCSharp/bin/Release/<platform>/publish/`.

2. **Build the Rust Project**:
   - Navigate to the `Rust` directory.
   - Run the build script using Cargo:
     ```bash
     cargo build
     ```
   - The build script (`build.rs`) will automatically copy the compiled C# library into the appropriate target directory (`target/debug` or `target/release`).

### Running the Rust Project

To run the Rust project, use the following command:

```bash
cargo run
```
This will execute the Rust program, which loads the C# native library and calls its functions.

## Implementing Rust to C# Interop

The next goal for this project is to enable calling Rust functions from C#. This will involve:

    Exposing Rust functions as C ABI functions.
    Using P/Invoke or similar techniques in C# to call Rust functions.
    Testing and validating the integration.

Stay tuned for updates as this feature is developed.

## Contributing

Contributions are welcome! If you'd like to help implement Rust to C# interop or improve the existing functionality, feel free to open an issue or submit a pull request.

## License

This project is licensed under the MIT License. See the LICENSE file for details.
