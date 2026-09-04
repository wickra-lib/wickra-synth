# Install the compiled package shared object plus the bundled C ABI library so
# the package is self-contained: on Windows wickra_synth.dll (matched by the
# *.dll glob and loaded via PATH in .onLoad); on Linux libwickra_synth.so
# (matched by the *.so SHLIB_EXT glob); on macOS libwickra_synth.dylib (added
# explicitly, since R package objects use the .so extension there too). The Unix
# rpath baked by configure ($ORIGIN / @loader_path) resolves it from this libs
# directory.
files <- unique(c(Sys.glob(paste0("*", SHLIB_EXT)), Sys.glob("libwickra_synth.dylib")))
dest <- file.path(R_PACKAGE_DIR, paste0("libs", R_ARCH))
dir.create(dest, recursive = TRUE, showWarnings = FALSE)
file.copy(files, dest, overwrite = TRUE)
if (file.exists("symbols.rds")) {
  file.copy("symbols.rds", dest, overwrite = TRUE)
}
