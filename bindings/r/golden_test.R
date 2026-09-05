## The cross-language golden corpus, seen from R.
##
## Deliberately not under tests/: this file walks up out of the package to find
## the repository's golden/ directory, and a test that ships must not reason
## about the repository it came from. `R CMD check` runs the tarball's tests in a
## temporary directory with nothing above it, so a walk there either finds
## something unrelated or has to be papered over with a skip -- and a skip that
## fires in every real check is a test that exists and never runs.
##
## So the package-local suite (tests/run_tests.R) covers the surface, and this
## file covers parity with the other nine languages. `.Rbuildignore` keeps it out
## of the tarball; ci.yml runs it from the repository root, where golden/ is
## simply there.
##
##   R CMD INSTALL bindings/r
##   Rscript bindings/r/golden_test.R

library(wickrasynth)

generate_cmd <- '{"cmd":"generate"}'

## Run from the repository root, so golden/ is one hop away. Stated as a
## requirement rather than searched for: if it is missing, the invocation is
## wrong and saying so beats silently checking nothing.
golden <- "golden"
if (!dir.exists(file.path(golden, "specs"))) {
  stop("run this from the repository root, where golden/specs exists")
}

spec_files <- sort(list.files(file.path(golden, "specs"), pattern = "[.]json$",
                              full.names = TRUE))
stopifnot(length(spec_files) > 0)

for (spec_path in spec_files) {
  name <- sub("[.]json$", "", basename(spec_path))
  expected <- trimws(paste(readLines(file.path(golden, "expected",
                                               paste0(name, ".json")),
                                     warn = FALSE), collapse = "\n"))
  spec_json <- paste(readLines(spec_path, warn = FALSE), collapse = "\n")
  got <- wksynth_command(wksynth_new(spec_json), generate_cmd)
  if (!identical(got, expected)) {
    stop(sprintf("%s: output is not byte-identical to the golden fixture", name))
  }
}

cat(sprintf("all %d golden fixtures match\n", length(spec_files)))
