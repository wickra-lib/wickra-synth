using System.IO;
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

    // The corpus is found by walking up from the test assembly rather than by a
    // fixed relative path: xUnit runs from bin/<config>/<tfm>, and that depth
    // changes with the target framework.
    private static string? FindGoldenDir()
    {
        DirectoryInfo? dir = new DirectoryInfo(AppContext.BaseDirectory);
        while (dir is not null)
        {
            string candidate = Path.Combine(dir.FullName, "golden", "specs");
            if (Directory.Exists(candidate))
            {
                return Path.Combine(dir.FullName, "golden");
            }
            dir = dir.Parent;
        }
        return null;
    }

    [Fact]
    public void GoldenCorpus_IsByteIdentical()
    {
        string? golden = FindGoldenDir();
        Assert.NotNull(golden);

        string[] specs = Directory.GetFiles(Path.Combine(golden!, "specs"), "*.json");
        Array.Sort(specs);
        Assert.NotEmpty(specs);

        foreach (string specPath in specs)
        {
            string name = Path.GetFileNameWithoutExtension(specPath);
            string expected = File.ReadAllText(Path.Combine(golden!, "expected", name + ".json")).Trim();
            using var synth = new Synth(File.ReadAllText(specPath));
            Assert.Equal(expected, synth.Command(SynthTests.GenerateCmd()));
        }
    }
}
