using System;
using System.Runtime.InteropServices;
using System.Text;

namespace NativeCSharp;

public static class NativeExports
{
    private static readonly Random _random = new();
    
    [UnmanagedCallersOnly(EntryPoint = nameof(GetInt))]
    public static int GetInt()
    {
        int randomValue = _random.Next(100);
        Console.WriteLine($"[.NET]: {nameof(GetInt)} called with a random value of '{randomValue}'.");
        return randomValue;
    }

    [UnmanagedCallersOnly(EntryPoint = nameof(GetString))]
    public static IntPtr GetString()
    {
        Console.WriteLine($"[.NET]: {nameof(GetString)} called.");
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
        Console.WriteLine($"[.NET]: String free called: '{Marshal.PtrToStringAnsi(ptr)}'.");
        Marshal.FreeHGlobal(ptr);
    }

    [UnmanagedCallersOnly(EntryPoint = nameof(Shutdown))]
    public static void Shutdown()
    {
        Console.WriteLine("[.NET]: Shutting down.");

        GC.Collect();
        GC.WaitForPendingFinalizers();
    }
}