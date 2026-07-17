#' The wickra-synth library version.
#' @return A version string.
#' @export
wksynth_version <- function() {
  .Call(C_wksynth_version)
}

#' Build a synth from a spec JSON.
#' @param spec_json A synth spec JSON string.
#' @return A `wickra_synth` handle (an external pointer).
#' @export
wksynth_new <- function(spec_json) {
  .Call(C_wksynth_new, spec_json)
}

#' Apply a command JSON and return the resulting response JSON.
#' @param synth A synth handle from [wksynth_new()].
#' @param cmd_json A command JSON string.
#' @return The response as a JSON string.
#' @export
wksynth_command <- function(synth, cmd_json) {
  .Call(C_wksynth_command, synth, cmd_json)
}
