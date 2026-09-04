// Optional C++ convenience layer over the wickra-synth C ABI (`wickra_synth.h`).
//
// The C ABI hands out an opaque handle that must be released exactly once with
// `wickra_synth_free`, and returns responses through a two-call length protocol
// (measure with `out = nullptr`, then write into a caller buffer).
// `wickra::synth::Synth` wraps both in a move-only RAII type:
//
//     #include "wickra_synth.hpp"
//
//     wickra::synth::Synth synth(spec_json);
//     std::string out = synth.command(R"({"cmd":"generate"})");
//     // synth is freed here
//
// This is header-only and adds no runtime cost beyond the C calls themselves.
// It never rewrites the response, so the bytes a C++ consumer sees are the bytes
// every other binding sees — which is what the golden corpus pins.

#ifndef WICKRA_SYNTH_HPP
#define WICKRA_SYNTH_HPP

#include "wickra_synth.h"

#include <stdexcept>
#include <string>
#include <utility>

namespace wickra {
namespace synth {

/// Thrown when the C ABI reports a failure: a rejected spec at construction, or
/// a negative error code from a command.
class Error : public std::runtime_error {
public:
    explicit Error(const std::string &what) : std::runtime_error(what) {}
};

/// Move-only RAII owner of a `WickraSynth *`.
class Synth {
public:
    /// Construct from a spec JSON string. Throws [`Error`] if the spec is
    /// rejected (the C ABI returns a null handle).
    explicit Synth(const std::string &spec_json)
        : handle_(wickra_synth_new(spec_json.c_str())) {
        if (handle_ == nullptr) {
            throw Error("wickra_synth_new rejected the spec");
        }
    }

    ~Synth() {
        if (handle_ != nullptr) {
            wickra_synth_free(handle_);
        }
    }

    Synth(const Synth &) = delete;
    Synth &operator=(const Synth &) = delete;

    Synth(Synth &&other) noexcept : handle_(std::exchange(other.handle_, nullptr)) {}

    Synth &operator=(Synth &&other) noexcept {
        if (this != &other) {
            if (handle_ != nullptr) {
                wickra_synth_free(handle_);
            }
            handle_ = std::exchange(other.handle_, nullptr);
        }
        return *this;
    }

    /// Apply a command JSON and return the response JSON.
    ///
    /// An invalid command or spec comes back in-band as
    /// `{"ok":false,"error":...}`; only a negative ABI code throws.
    std::string command(const std::string &cmd_json) {
        const int32_t len = wickra_synth_command(handle_, cmd_json.c_str(), nullptr, 0);
        if (len < 0) {
            throw Error("wickra_synth_command failed with code " + std::to_string(len));
        }
        std::string out(static_cast<std::size_t>(len), '\0');
        // `out.size() + 1` counts the NUL the ABI writes; std::string keeps its
        // own terminator past back(), so writing there is well defined.
        const int32_t written =
            wickra_synth_command(handle_, cmd_json.c_str(), &out[0], out.size() + 1);
        if (written < 0) {
            throw Error("wickra_synth_command failed with code " + std::to_string(written));
        }
        return out;
    }

    /// The raw handle, for calls this wrapper does not cover.
    WickraSynth *get() const noexcept { return handle_; }

    /// The library version.
    static std::string version() { return std::string(wickra_synth_version()); }

private:
    WickraSynth *handle_;
};

}  // namespace synth
}  // namespace wickra

#endif  // WICKRA_SYNTH_HPP
