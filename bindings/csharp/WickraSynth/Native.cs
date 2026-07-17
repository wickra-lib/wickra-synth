using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;

namespace Wickra.Synth;

/// <summary>Raw P/Invoke surface for the wickra-synth C ABI.</summary>
internal static partial class Native
{
    internal const string Lib = "wickra_synth";

    /// <summary>
    /// Construct a synth handle from a spec JSON (NUL-terminated UTF-8).
    /// Returns null if the spec is null, not valid UTF-8, or not a valid spec.
    /// </summary>
    [LibraryImport(Lib)]
    [UnmanagedCallConv(CallConvs = [typeof(CallConvCdecl)])]
    internal static partial IntPtr wickra_synth_new(byte[] specUtf8);

    /// <summary>Free a synth handle.</summary>
    [LibraryImport(Lib)]
    [UnmanagedCallConv(CallConvs = [typeof(CallConvCdecl)])]
    internal static partial void wickra_synth_free(IntPtr handle);

    /// <summary>
    /// Apply a command JSON (NUL-terminated UTF-8), writing the response into a
    /// caller-owned buffer. Returns the response length, or a negative error code.
    /// </summary>
    [LibraryImport(Lib)]
    [UnmanagedCallConv(CallConvs = [typeof(CallConvCdecl)])]
    internal static partial int wickra_synth_command(IntPtr handle, byte[] cmdUtf8, byte[]? outBuf, nuint cap);

    /// <summary>The library version as a static NUL-terminated string.</summary>
    [LibraryImport(Lib)]
    [UnmanagedCallConv(CallConvs = [typeof(CallConvCdecl)])]
    internal static partial IntPtr wickra_synth_version();
}
