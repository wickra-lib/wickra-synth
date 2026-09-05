\name{wksynth_version}
\alias{wksynth_version}
\title{The wickra-synth library version}
\usage{
wksynth_version()
}
\value{
  A length-one character vector: the version of the native library this package
  is linked against.
}
\description{
  Reports the version of the C ABI library, not of the R package. The two are
  released together, so a mismatch means the bundled library did not come from
  this release.
}
\examples{
wksynth_version()
}
