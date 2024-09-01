using System;
using System.Runtime.InteropServices;

namespace NativeCSharp;

public static class NativeExports
{
    [UnmanagedCallersOnly(EntryPoint = nameof(GetInt))]
    public static int GetInt()
    {
        Console.WriteLine($"{nameof(GetInt)} called in csharp native code");
        return 42;
    }
}