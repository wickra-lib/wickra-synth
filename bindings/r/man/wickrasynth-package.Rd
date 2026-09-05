\name{wickrasynth-package}
\alias{wickrasynth-package}
\alias{wickrasynth}
\docType{package}
\title{Deterministic Synthetic Market Microstructure}
\description{
  R bindings for wickra-synth over its C ABI hub. A seeded \code{GenSpec}
  produces byte-identical OHLCV candles, order-book snapshots, trades and
  funding: the same seed yields the same bytes here as in the Rust core and in
  every other language binding, which is what the project's golden corpus
  checks.

  The whole surface is three functions over one JSON boundary. Build a handle
  with \code{\link{wksynth_new}}, drive it with \code{\link{wksynth_command}},
  and read the version with \code{\link{wksynth_version}}.
}
\details{
  The native library is resolved at build time by \code{configure}: it
  downloads the prebuilt C ABI matching this package's version and bundles it
  beside the compiled object. Set \code{WKSYNTH_INC} and \code{WKSYNTH_LIB} to
  build against a locally built one instead.
}
\author{
  kingchenc \email{support@wickra.org}
}
\references{
  \url{https://github.com/wickra-lib/wickra-synth}
}
\keyword{package}
