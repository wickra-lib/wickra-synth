using System.Text.Json;
using System.Text.Json.Nodes;
using Wickra.Synth;
using Xunit;

namespace WickraSynth.Tests;

// The cross-language golden invariant seen from C#: the same seed yields
// byte-identical output across calls, and the streamed event list carries the
// same candles as the batch generate. The response bytes are what every other
// binding produces too, because the whole generator lives once in the Rust core
// and this binding forwards its JSON verbatim.
public class GoldenTests
{
    [Fact]
    public void Generate_IsByteIdenticalAcrossCalls()
    {
        using var a = new Synth(SynthTests.Spec);
        using var b = new Synth(SynthTests.Spec);
        Assert.Equal(a.Command(SynthTests.GenerateCmd()), b.Command(SynthTests.GenerateCmd()));
    }

    [Fact]
    public void StreamCandles_MatchBatch()
    {
        using var batchSynth = new Synth(SynthTests.Spec);
        JsonNode batch = JsonNode.Parse(batchSynth.Command(SynthTests.GenerateCmd()))!;
        JsonArray batchCandles = batch["candles"]!.AsArray();

        using var streamSynth = new Synth(SynthTests.Spec);
        JsonNode stream = JsonNode.Parse(streamSynth.Command("{\"cmd\":\"generate_stream\"}"))!;

        var streamedCandles = new List<string>();
        foreach (JsonNode? ev in stream["events"]!.AsArray())
        {
            if (ev!["type"]!.GetValue<string>() == "candle")
            {
                streamedCandles.Add(ev["candle"]!.ToJsonString());
            }
        }

        Assert.Equal(batchCandles.Count, streamedCandles.Count);
        for (int i = 0; i < batchCandles.Count; i++)
        {
            Assert.Equal(batchCandles[i]!.ToJsonString(), streamedCandles[i]);
        }
    }
}
