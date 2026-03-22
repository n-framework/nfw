using NFramework.Core.CLI.Abstractions;

namespace NFramework.NFW.CLI;

/// <summary>
/// Marker class for the source generator.
/// The generator adds a static Create() factory method that produces the Spectre.Console command app.
/// </summary>
[CliApplication("nfw")]
public sealed partial class NfwCliApplication;
