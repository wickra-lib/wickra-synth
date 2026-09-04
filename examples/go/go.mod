module wickra-synth-example

go 1.23

require github.com/wickra-lib/wickra-synth/bindings/go v0.0.0

// The binding lives in this repository, not on the module proxy: the version
// above is a placeholder that no proxy can resolve, so the example builds
// against the tree it ships in.
replace github.com/wickra-lib/wickra-synth/bindings/go => ../../bindings/go
