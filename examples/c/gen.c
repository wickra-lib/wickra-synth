/* A runnable C example: generate synthetic microstructure through the
 * wickra-synth C ABI and print the raw JSON output. Every language example
 * uses the same seed and prints the same candles. */
#include <stdio.h>
#include <stdlib.h>

#include "wickra_synth.h"

static const char *SPEC =
    "{\"seed\":42,\"bars\":20,\"start_price\":100.0,"
    "\"regimes\":[{\"kind\":\"trend\",\"len\":20,\"drift\":0.002,\"vol\":0.01}],"
    "\"microstructure\":{\"book_depth\":5,\"spread_bps\":4.0,\"trade_rate\":8.0,"
    "\"funding\":{\"interval_bars\":8,\"base_rate\":0.0001,\"sensitivity\":0.5}}}";

static const char *CMD = "{\"cmd\":\"generate\"}";

int main(void) {
    WickraSynth *synth = wickra_synth_new(SPEC);
    if (!synth) {
        fprintf(stderr, "failed to build synth\n");
        return 1;
    }

    /* Length-out protocol: learn the length, then read into a caller buffer. */
    int len = wickra_synth_command(synth, CMD, NULL, 0);
    if (len < 0) {
        fprintf(stderr, "command failed: code %d\n", len);
        wickra_synth_free(synth);
        return 1;
    }
    char *buf = (char *)malloc((size_t)len + 1);
    if (!buf) {
        wickra_synth_free(synth);
        return 1;
    }
    wickra_synth_command(synth, CMD, buf, (size_t)len + 1);

    printf("wickra-synth %s\n", wickra_synth_version());
    printf("output: %s\n", buf);

    free(buf);
    wickra_synth_free(synth);
    return 0;
}
