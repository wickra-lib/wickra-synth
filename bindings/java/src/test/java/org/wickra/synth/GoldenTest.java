package org.wickra.synth;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertTrue;

import org.junit.jupiter.api.Test;

// The cross-language golden invariant seen from Java: the same seed yields
// byte-identical output across calls, and each candle in the streamed event list
// is byte-identical to the corresponding batch candle. The response bytes are
// what every other binding produces too, because the whole generator lives once
// in the Rust core and this binding forwards its JSON verbatim.
class GoldenTest {
    @Test
    void generateIsByteIdenticalAcrossCalls() {
        try (Synth a = new Synth(SynthTest.SPEC);
                Synth b = new Synth(SynthTest.SPEC)) {
            assertEquals(a.command(SynthTest.generateCmd()), b.command(SynthTest.generateCmd()));
        }
    }

    @Test
    void streamCandlesAreByteIdenticalToBatch() {
        String batch;
        String stream;
        try (Synth batchSynth = new Synth(SynthTest.SPEC)) {
            batch = batchSynth.command(SynthTest.generateCmd());
        }
        try (Synth streamSynth = new Synth(SynthTest.SPEC)) {
            stream = streamSynth.command("{\"cmd\":\"generate_stream\"}");
        }

        // Candle objects are flat (ts, open, high, low, close, volume), so the
        // batch "candles":[{...},{...}] array can be split on the object boundary.
        int start = batch.indexOf("\"candles\":[") + "\"candles\":[".length();
        int end = batch.indexOf(']', start);
        String[] candles = batch.substring(start, end).split("(?<=\\}),(?=\\{)");
        assertEquals(8, candles.length, batch);
        for (String candle : candles) {
            // Each batch candle appears verbatim as a streamed candle event body.
            assertTrue(stream.contains("\"candle\":" + candle), candle + "\nnot in\n" + stream);
        }
    }
}
