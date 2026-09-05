/* Streaming equals batch, seen from C.
 *
 * `generate` returns the whole run at once; `generate_stream` returns the same
 * data as an ordered event list, drawn in the same order from the same seed. If
 * the two ever diverge, a consumer reading the stream sees a different market
 * from one reading the batch, for the same spec -- and every golden fixture
 * would still pass, because the corpus pins only the batch shape.
 *
 * Checked here rather than only in Rust because the C ABI hands both back
 * through the same length-out buffer protocol, and a buffer bug that truncated
 * one and not the other would look exactly like a divergence.
 *
 * Substring matching rather than JSON parsing on purpose: candle objects are
 * flat, and the stream embeds each one verbatim as `"candle":{...}`. Pulling in
 * a JSON parser to prove that the bytes are the same bytes would be the wrong
 * tool. */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "wickra_synth.h"

static const char *SPEC =
    "{\"seed\":42,\"bars\":8,\"start_price\":100.0,"
    "\"regimes\":[{\"kind\":\"trend\",\"len\":8,\"drift\":0.002,\"vol\":0.01}],"
    "\"microstructure\":{\"book_depth\":3,\"spread_bps\":4.0,\"trade_rate\":3.0}}";

/* Run one command and return its response in a fresh heap buffer, or NULL. */
static char *run(WickraSynth *synth, const char *cmd) {
    int len = wickra_synth_command(synth, cmd, NULL, 0);
    if (len < 0) {
        fprintf(stderr, "command failed with code %d\n", len);
        return NULL;
    }
    char *buf = (char *)malloc((size_t)len + 1);
    if (!buf) {
        return NULL;
    }
    wickra_synth_command(synth, cmd, buf, (size_t)len + 1);
    return buf;
}

int main(void) {
    WickraSynth *batch_synth = wickra_synth_new(SPEC);
    WickraSynth *stream_synth = wickra_synth_new(SPEC);
    if (!batch_synth || !stream_synth) {
        fprintf(stderr, "failed to build synth\n");
        wickra_synth_free(batch_synth);
        wickra_synth_free(stream_synth);
        return 1;
    }

    char *batch = run(batch_synth, "{\"cmd\":\"generate\"}");
    char *stream = run(stream_synth, "{\"cmd\":\"generate_stream\"}");
    int rc = 1;
    if (!batch || !stream) {
        goto done;
    }

    /* Slice the batch "candles":[{...},{...}] array on the object boundary. */
    const char *key = "\"candles\":[";
    char *start = strstr(batch, key);
    if (!start) {
        fprintf(stderr, "batch output carries no candles array\n");
        goto done;
    }
    start += strlen(key);
    char *end = strchr(start, ']');
    if (!end) {
        fprintf(stderr, "batch candles array is unterminated\n");
        goto done;
    }

    int checked = 0;
    char *cursor = start;
    while (cursor < end) {
        char *close = strstr(cursor, "}");
        if (!close || close > end) {
            break;
        }
        size_t span = (size_t)(close - cursor) + 1;
        char *candle = (char *)malloc(span + 1);
        if (!candle) {
            goto done;
        }
        memcpy(candle, cursor, span);
        candle[span] = '\0';

        /* Each batch candle must appear verbatim as a streamed candle event. */
        char *needle = (char *)malloc(span + strlen("\"candle\":") + 1);
        if (!needle) {
            free(candle);
            goto done;
        }
        sprintf(needle, "\"candle\":%s", candle);
        int found = strstr(stream, needle) != NULL;
        if (!found) {
            fprintf(stderr, "candle %d is not in the stream: %s\n", checked, candle);
            free(needle);
            free(candle);
            goto done;
        }
        free(needle);
        free(candle);
        checked++;

        cursor = close + 1;
        if (*cursor == ',') {
            cursor++;
        }
    }

    if (checked != 8) {
        fprintf(stderr, "expected 8 candles, matched %d\n", checked);
        goto done;
    }
    printf("streaming equals batch across %d candles\n", checked);
    rc = 0;

done:
    free(batch);
    free(stream);
    wickra_synth_free(batch_synth);
    wickra_synth_free(stream_synth);
    return rc;
}
