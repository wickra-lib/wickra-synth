package org.wickra.synth;

import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertThrows;
import static org.junit.jupiter.api.Assertions.assertTrue;

import org.junit.jupiter.api.Test;

class SynthTest {
    static final String SPEC =
            "{\"seed\":42,\"bars\":8,\"start_price\":100.0,"
                    + "\"regimes\":[{\"kind\":\"trend\",\"len\":8,\"drift\":0.002,\"vol\":0.01}],"
                    + "\"microstructure\":{\"book_depth\":3,\"spread_bps\":4.0,\"trade_rate\":3.0}}";

    static String generateCmd() {
        return "{\"cmd\":\"generate\"}";
    }

    @Test
    void versionIsNonEmpty() {
        assertFalse(Synth.version().isEmpty());
    }

    @Test
    void generateReturnsCandlesAndBook() {
        try (Synth synth = new Synth(SPEC)) {
            String out = synth.command(generateCmd());
            // Eight bars each yield one candle and one book snapshot.
            assertTrue(out.contains("\"candles\""), out);
            assertTrue(out.contains("\"book_snapshots\""), out);
            assertTrue(out.contains("\"ts\":1700000000"), out);
        }
    }

    @Test
    void invalidSpecThrows() {
        assertThrows(IllegalArgumentException.class, () -> new Synth("{ not valid json"));
    }

    @Test
    void unknownCommandIsInBandError() {
        try (Synth synth = new Synth(SPEC)) {
            // The C ABI hub folds a domain error into {"ok":false,...} JSON, so an
            // unknown command surfaces in-band rather than as an exception.
            String raw = synth.command("{\"cmd\":\"nope\"}");
            assertTrue(raw.contains("\"ok\":false"), raw);
        }
    }
}
