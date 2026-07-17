using System.Text.Json;
using Wickra.Synth;
using Xunit;

namespace WickraSynth.Tests;

public class SynthTests
{
    internal const string Spec =
        "{\"seed\":42,\"bars\":8,\"start_price\":100.0," +
        "\"regimes\":[{\"kind\":\"trend\",\"len\":8,\"drift\":0.002,\"vol\":0.01}]," +
        "\"microstructure\":{\"book_depth\":3,\"spread_bps\":4.0,\"trade_rate\":3.0}}";

    internal static string GenerateCmd() => "{\"cmd\":\"generate\"}";

    [Fact]
    public void Version_IsNonEmpty()
    {
        Assert.False(string.IsNullOrEmpty(Synth.Version()));
    }

    [Fact]
    public void Generate_ReturnsCandlesAndBook()
    {
        using var synth = new Synth(Spec);
        JsonElement outp = JsonDocument.Parse(synth.Command(GenerateCmd())).RootElement;

        Assert.Equal(8, outp.GetProperty("candles").GetArrayLength());
        Assert.Equal(8, outp.GetProperty("book_snapshots").GetArrayLength());
    }

    [Fact]
    public void InvalidSpec_Throws()
    {
        Assert.Throws<ArgumentException>(() => new Synth("{ not valid json"));
    }

    [Fact]
    public void UnknownCommand_IsInBandError()
    {
        using var synth = new Synth(Spec);
        // The C ABI hub folds a domain error into {"ok":false,...} JSON, so an
        // unknown command surfaces in-band rather than as an exception.
        string raw = synth.Command("{\"cmd\":\"nope\"}");
        Assert.Contains("\"ok\":false", raw);
    }
}
