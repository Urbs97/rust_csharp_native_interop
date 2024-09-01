using System;
using System.Runtime.InteropServices;
using System.Text;

namespace NativeCSharp;

public static class NativeExports
{
    [UnmanagedCallersOnly(EntryPoint = nameof(GetInt))]
    public static int GetInt()
    {
        Console.WriteLine($"{nameof(GetInt)} called in C# native code");
        return 42;
    }

    [UnmanagedCallersOnly(EntryPoint = nameof(GetString))]
    public static IntPtr GetString()
    {
        Console.WriteLine($"{nameof(GetString)} called in C# native code");
        const string managedString = "This is a test string from C#.";

        // Convert the managed string to a UTF-8 encoded byte array
        byte[] utf8Bytes = Encoding.UTF8.GetBytes(managedString);

        // Allocate unmanaged memory for the string
        IntPtr unmanagedString = Marshal.AllocHGlobal(utf8Bytes.Length + 1);

        // Copy the UTF-8 bytes to the unmanaged memory
        Marshal.Copy(utf8Bytes, 0, unmanagedString, utf8Bytes.Length);

        // Null-terminate the string
        Marshal.WriteByte(unmanagedString + utf8Bytes.Length, 0);

        return unmanagedString;
    }

    [UnmanagedCallersOnly(EntryPoint = nameof(FreeString))]
    public static void FreeString(IntPtr ptr)
    {
        Console.WriteLine($"String free: {Marshal.PtrToStringAnsi(ptr)}");
        Marshal.FreeHGlobal(ptr);
    }

    [UnmanagedCallersOnly(EntryPoint = nameof(Shutdown))]
    public static void Shutdown()
    {
        Console.WriteLine("Calling Shutdown in C#");

        GC.Collect();
        GC.WaitForPendingFinalizers();
    }
}