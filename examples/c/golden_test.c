/* The cross-language golden invariant seen from C: replay every spec in
 * golden/specs through the C ABI and assert the response is byte-for-byte the
 * blessed golden/expected output. Comparing against the committed corpus (not
 * just against a second call in the same process) is what makes this a
 * cross-language check rather than a tautology.
 *
 * WICKRA_SYNTH_GOLDEN_DIR and WICKRA_SYNTH_GOLDEN_NAMES are baked in by CMake,
 * so the test runs from any working directory and picks up a new fixture
 * without an edit here. */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "wickra_synth.h"

static const char *NAMES[] = {WICKRA_SYNTH_GOLDEN_NAMES};

static const char *CMD = "{\"cmd\":\"generate\"}";

/* Read a whole file into a NUL-terminated heap buffer. Returns NULL on error. */
static char *read_file(const char *path) {
    FILE *f = fopen(path, "rb");
    if (!f) {
        return NULL;
    }
    if (fseek(f, 0, SEEK_END) != 0) {
        fclose(f);
        return NULL;
    }
    long size = ftell(f);
    if (size < 0 || fseek(f, 0, SEEK_SET) != 0) {
        fclose(f);
        return NULL;
    }
    char *buf = (char *)malloc((size_t)size + 1);
    if (!buf) {
        fclose(f);
        return NULL;
    }
    size_t got = fread(buf, 1, (size_t)size, f);
    fclose(f);
    buf[got] = '\0';
    return buf;
}

/* Strip the trailing newline the blessing step leaves on expected/*.json. */
static void rstrip(char *s) {
    size_t n = strlen(s);
    while (n > 0 && (s[n - 1] == '\n' || s[n - 1] == '\r')) {
        s[--n] = '\0';
    }
}

static int check_one(const char *name) {
    char spec_path[512];
    char expected_path[512];
    snprintf(spec_path, sizeof spec_path, "%s/specs/%s.json", WICKRA_SYNTH_GOLDEN_DIR, name);
    snprintf(expected_path, sizeof expected_path, "%s/expected/%s.json", WICKRA_SYNTH_GOLDEN_DIR,
             name);

    char *spec = read_file(spec_path);
    if (!spec) {
        fprintf(stderr, "cannot read %s\n", spec_path);
        return 1;
    }
    char *expected = read_file(expected_path);
    if (!expected) {
        fprintf(stderr, "cannot read %s\n", expected_path);
        free(spec);
        return 1;
    }
    rstrip(expected);

    WickraSynth *synth = wickra_synth_new(spec);
    if (!synth) {
        fprintf(stderr, "%s: failed to build synth\n", name);
        free(spec);
        free(expected);
        return 1;
    }

    int len = wickra_synth_command(synth, CMD, NULL, 0);
    if (len < 0) {
        fprintf(stderr, "%s: command failed with code %d\n", name, len);
        wickra_synth_free(synth);
        free(spec);
        free(expected);
        return 1;
    }
    char *got = (char *)malloc((size_t)len + 1);
    if (!got) {
        wickra_synth_free(synth);
        free(spec);
        free(expected);
        return 1;
    }
    wickra_synth_command(synth, CMD, got, (size_t)len + 1);

    int rc = 0;
    if (strcmp(got, expected) != 0) {
        fprintf(stderr, "%s: output is not byte-identical to the golden fixture\n", name);
        rc = 1;
    } else {
        printf("%s: byte-identical (%d bytes)\n", name, len);
    }

    free(got);
    wickra_synth_free(synth);
    free(spec);
    free(expected);
    return rc;
}

int main(void) {
    size_t count = sizeof NAMES / sizeof NAMES[0];
    if (count == 0) {
        fprintf(stderr, "no golden fixtures were configured\n");
        return 1;
    }
    int failures = 0;
    for (size_t i = 0; i < count; i++) {
        failures += check_one(NAMES[i]);
    }
    if (failures != 0) {
        fprintf(stderr, "%d of %zu golden fixtures did not match\n", failures, count);
        return 1;
    }
    printf("all %zu golden fixtures match\n", count);
    return 0;
}
