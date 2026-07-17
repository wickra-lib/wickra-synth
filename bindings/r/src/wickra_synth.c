/* R .Call glue for the wickra-synth C ABI hub. */
#include <R.h>
#include <Rinternals.h>
#include <R_ext/Rdynload.h>
#include <stddef.h>
#include "wickra_synth.h"

/* --- handle lifetime ----------------------------------------------------- */

static void wksynth_finalize(SEXP ext) {
    WickraSynth *h = (WickraSynth *)R_ExternalPtrAddr(ext);
    if (h) {
        wickra_synth_free(h);
    }
    R_ClearExternalPtr(ext);
}

static WickraSynth *handle_of(SEXP ext) {
    WickraSynth *h = (WickraSynth *)R_ExternalPtrAddr(ext);
    if (!h) {
        Rf_error("wickra-synth: handle is closed");
    }
    return h;
}

/* --- exported .Call entries ---------------------------------------------- */

SEXP wksynth_version(void) {
    return Rf_mkString(wickra_synth_version());
}

SEXP wksynth_new(SEXP spec_json) {
    const char *spec = CHAR(STRING_ELT(spec_json, 0));
    WickraSynth *h = wickra_synth_new(spec);
    if (!h) {
        Rf_error("wickra-synth: invalid spec");
    }
    SEXP ext = PROTECT(R_MakeExternalPtr(h, R_NilValue, R_NilValue));
    R_RegisterCFinalizerEx(ext, wksynth_finalize, TRUE);
    UNPROTECT(1);
    return ext;
}

SEXP wksynth_command(SEXP ext, SEXP cmd_json) {
    WickraSynth *h = handle_of(ext);
    const char *cmd = CHAR(STRING_ELT(cmd_json, 0));

    /* Length-out protocol: learn the length, then read into a caller buffer.
       Domain errors come back in-band as {"ok":false,...} JSON, not a negative
       code; only unusable arguments / a caught panic return < 0. */
    int len = wickra_synth_command(h, cmd, NULL, 0);
    if (len < 0) {
        Rf_error("wickra-synth: command failed (code %d)", len);
    }
    char *buf = (char *)R_alloc((size_t)len + 1, 1);
    wickra_synth_command(h, cmd, buf, (size_t)len + 1);
    return Rf_mkString(buf);
}

/* --- registration -------------------------------------------------------- */

static const R_CallMethodDef CallEntries[] = {
    {"wksynth_version", (DL_FUNC)&wksynth_version, 0},
    {"wksynth_new", (DL_FUNC)&wksynth_new, 1},
    {"wksynth_command", (DL_FUNC)&wksynth_command, 2},
    {NULL, NULL, 0}};

void R_init_wickrasynth(DllInfo *dll) {
    R_registerRoutines(dll, NULL, CallEntries, NULL, NULL);
    R_useDynamicSymbols(dll, FALSE);
}
