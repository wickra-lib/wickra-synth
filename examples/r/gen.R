# A runnable R example: generate synthetic microstructure through the binding.
#
#   R CMD INSTALL bindings/r
#   Rscript examples/r/gen.R
#
# Every language example uses the same seed and prints the same candles.
library(wickrasynth)

spec <- paste0(
  '{"seed":42,"bars":20,"start_price":100.0,',
  '"regimes":[{"kind":"trend","len":20,"drift":0.002,"vol":0.01}],',
  '"microstructure":{"book_depth":5,"spread_bps":4.0,"trade_rate":8.0,',
  '"funding":{"interval_bars":8,"base_rate":0.0001,"sensitivity":0.5}}}'
)

synth <- wksynth_new(spec)
response <- wksynth_command(synth, '{"cmd":"generate"}')

cat(sprintf("wickra-synth %s\n", wksynth_version()))
cat(response, "\n")
