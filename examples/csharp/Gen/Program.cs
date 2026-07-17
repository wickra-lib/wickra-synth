// A runnable C# example: generate synthetic microstructure and print the first
// three candles.
//
//   dotnet run --project examples/csharp/Gen
//
// Every language example uses the same seed and prints the same candles.
using System.Text.Json;
using Wickra.Synth;

const string spec = """
    {"seed":42,"bars":20,"start_price":100.0,
     "regimes":[{"kind":"trend","len":20,"drift":0.002,"vol":0.01}],
     "microstructure":{"book_depth":5,"spread_bps":4.0,"trade_rate":8.0,
        "funding":{"interval_bars":8,"base_rate":0.0001,"sensitivity":0.5}}}
    """;

using var synth = new Synth(spec);
var raw = synth.Command("""{"cmd":"generate"}""");
using var doc = JsonDocument.Parse(raw);
var candles = doc.RootElement.GetProperty("candles");

Console.WriteLine($"wickra-synth {Synth.Version()}");
Console.WriteLine($"bars: {candles.GetArrayLength()}");
Console.WriteLine("first 3 candles:");
for (var i = 0; i < 3 && i < candles.GetArrayLength(); i++)
{
    Console.WriteLine($"  {candles[i].GetRawText()}");
}
