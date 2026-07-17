// A runnable C++ example: generate synthetic microstructure through the
// wickra-synth C ABI. Every language example uses the same seed and prints the
// same candles.
#include <cstdlib>
#include <iostream>
#include <string>
#include <vector>

#include "wickra_synth.h"

namespace {
constexpr const char *kSpec =
    R"({"seed":42,"bars":20,"start_price":100.0,)"
    R"("regimes":[{"kind":"trend","len":20,"drift":0.002,"vol":0.01}],)"
    R"("microstructure":{"book_depth":5,"spread_bps":4.0,"trade_rate":8.0,)"
    R"("funding":{"interval_bars":8,"base_rate":0.0001,"sensitivity":0.5}}})";
constexpr const char *kCmd = R"({"cmd":"generate"})";
}  // namespace

int main() {
    WickraSynth *synth = wickra_synth_new(kSpec);
    if (synth == nullptr) {
        std::cerr << "failed to build synth\n";
        return 1;
    }

    const int len = wickra_synth_command(synth, kCmd, nullptr, 0);
    if (len < 0) {
        std::cerr << "command failed: code " << len << "\n";
        wickra_synth_free(synth);
        return 1;
    }
    std::vector<char> buf(static_cast<std::size_t>(len) + 1);
    wickra_synth_command(synth, kCmd, buf.data(), buf.size());

    std::cout << "wickra-synth " << wickra_synth_version() << "\n";
    std::cout << "output: " << std::string(buf.data()) << "\n";

    wickra_synth_free(synth);
    return 0;
}
