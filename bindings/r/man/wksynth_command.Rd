\name{wksynth_command}
\alias{wksynth_command}
\title{Apply a command and return the response}
\usage{
wksynth_command(synth, cmd_json)
}
\arguments{
  \item{synth}{A handle from \code{\link{wksynth_new}}.}
  \item{cmd_json}{A character vector of length one holding the command JSON,
    for example \code{'\{"cmd":"generate"\}'} or
    \code{'\{"cmd":"generate_stream"\}'}.}
}
\value{
  A length-one character vector holding the response JSON, returned byte for
  byte as the core produced it.
}
\description{
  The single entry point every wickra-synth binding forwards to. Four commands
  exist: \code{version}, \code{set_spec}, \code{generate} and
  \code{generate_stream}.

  This function does not raise on a bad command or a bad spec. Failure comes
  back in the response as \code{\{"ok":false,"error":...\}}, so a caller checks
  the parsed result rather than wrapping the call.
}
\examples{
spec <- paste0(
  '{"seed":42,"bars":8,"start_price":100.0,',
  '"regimes":[{"kind":"trend","len":8,"drift":0.002,"vol":0.01}],',
  '"microstructure":{"book_depth":3,"spread_bps":4.0,"trade_rate":3.0}}'
)
synth <- wksynth_new(spec)
out <- wksynth_command(synth, '{"cmd":"generate"}')
}
\seealso{
  \code{\link{wksynth_new}}
}
