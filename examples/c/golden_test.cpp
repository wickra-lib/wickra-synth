// The cross-language golden invariant seen from C++: replay every spec in
// golden/specs through the C ABI and assert the response is byte-for-byte the
// blessed golden/expected output. The C++ hull is a thin RAII wrapper over the
// same four C functions, so this also proves the wrapper does not touch bytes.
//
// WICKRA_SYNTH_GOLDEN_DIR and WICKRA_SYNTH_GOLDEN_NAMES are baked in by CMake,
// so the test runs from any working directory and picks up a new fixture
// without an edit here.
#include <cstdio>
#include <fstream>
#include <iostream>
#include <sstream>
#include <string>
#include <vector>

#include "wickra_synth.hpp"

namespace {

const std::vector<std::string> kNames = {WICKRA_SYNTH_GOLDEN_NAMES};

std::string read_file(const std::string &path) {
    std::ifstream in(path, std::ios::binary);
    if (!in) {
        throw std::runtime_error("cannot read " + path);
    }
    std::ostringstream ss;
    ss << in.rdbuf();
    return ss.str();
}

// Strip the trailing newline the blessing step leaves on expected/*.json.
std::string rstrip(std::string s) {
    while (!s.empty() && (s.back() == '\n' || s.back() == '\r')) {
        s.pop_back();
    }
    return s;
}

bool check_one(const std::string &name) {
    const std::string dir = WICKRA_SYNTH_GOLDEN_DIR;
    const std::string spec = read_file(dir + "/specs/" + name + ".json");
    const std::string expected = rstrip(read_file(dir + "/expected/" + name + ".json"));

    wickra::synth::Synth synth(spec);
    const std::string got = synth.command("{\"cmd\":\"generate\"}");

    if (got != expected) {
        std::cerr << name << ": output is not byte-identical to the golden fixture\n";
        return false;
    }
    std::cout << name << ": byte-identical (" << got.size() << " bytes)\n";
    return true;
}

}  // namespace

int main() {
    if (kNames.empty()) {
        std::cerr << "no golden fixtures were configured\n";
        return 1;
    }
    std::size_t failures = 0;
    try {
        for (const std::string &name : kNames) {
            if (!check_one(name)) {
                ++failures;
            }
        }
    } catch (const std::exception &e) {
        std::cerr << "golden test failed: " << e.what() << "\n";
        return 1;
    }
    if (failures != 0) {
        std::cerr << failures << " of " << kNames.size() << " golden fixtures did not match\n";
        return 1;
    }
    std::cout << "all " << kNames.size() << " golden fixtures match\n";
    return 0;
}
