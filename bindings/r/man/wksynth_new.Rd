\name{wksynth_new}
\alias{wksynth_new}
\title{Build a synth from a spec}
\usage{
wksynth_new(spec_json)
}
\arguments{
  \item{spec_json}{A character vector of length one holding a \code{GenSpec} as
    JSON. It may also be \code{"\{\}"} or the empty string, which builds a handle
    with no spec; a command must then carry its own \code{"spec"} field.}
}
\value{
  A \code{wickra_synth} handle: an external pointer whose finalizer frees the
  native object, so there is nothing to close by hand.
}
\description{
  Parses and validates the spec. An invalid spec is a hard error here rather
  than an in-band one later, which is the one place this binding raises: every
  command error comes back inside the returned JSON instead.
}
\examples{
spec <- paste0(
  '{"seed":42,"bars":8,"start_price":100.0,',
  '"regimes":[{"kind":"trend","len":8,"drift":0.002,"vol":0.01}],',
  '"microstructure":{"book_depth":3,"spread_bps":4.0,"trade_rate":3.0}}'
)
synth <- wksynth_new(spec)
}
\seealso{
  \code{\link{wksynth_command}}
}
