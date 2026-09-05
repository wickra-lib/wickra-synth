## Plain-R tests for the wickra-synth R binding (no testthat dependency).
## Mirrors the Rust/Python/Node/Go/C#/Java tests and doubles as the completeness
## guard: it exercises the full public surface (version + new + command).

library(wickrasynth)

spec <- paste0(
  '{"seed":42,"bars":8,"start_price":100.0,',
  '"regimes":[{"kind":"trend","len":8,"drift":0.002,"vol":0.01}],',
  '"microstructure":{"book_depth":3,"spread_bps":4.0,"trade_rate":3.0}}'
)

generate_cmd <- function() {
  '{"cmd":"generate"}'
}

## version
stopifnot(nzchar(wksynth_version()))

## generate returns candles and book snapshots
synth <- wksynth_new(spec)
out <- wksynth_command(synth, generate_cmd())
stopifnot(grepl('"candles"', out, fixed = TRUE))
stopifnot(grepl('"book_snapshots"', out, fixed = TRUE))
stopifnot(grepl('"ts":1700000000', out, fixed = TRUE))

## generate is byte-identical across synths (the cross-language golden core)
synth2 <- wksynth_new(spec)
out2 <- wksynth_command(synth2, generate_cmd())
stopifnot(identical(out, out2))

## an invalid spec is a hard error at construction
err <- tryCatch(wksynth_new("{ not valid json"), error = function(e) e)
stopifnot(inherits(err, "error"))

## an unknown command is an in-band error, not a hard error
inband <- wksynth_command(synth, '{"cmd":"nope"}')
stopifnot(grepl('"ok":false', inband, fixed = TRUE))

## the streamed event list carries the same candles as the batch generate
stream <- wksynth_command(wksynth_new(spec), '{"cmd":"generate_stream"}')
## Candle objects are flat, so each batch candle appears verbatim as a streamed
## candle event body.
candles_section <- sub('^.*"candles":\\[', "", out)
candles_section <- sub('\\].*$', "", candles_section)
candle_objs <- strsplit(candles_section, "(?<=\\}),(?=\\{)", perl = TRUE)[[1]]
stopifnot(length(candle_objs) == 8)
for (candle in candle_objs) {
  stopifnot(grepl(paste0('"candle":', candle), stream, fixed = TRUE))
}

cat("wickra-synth R tests passed\n")
