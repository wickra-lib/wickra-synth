# Build and run the C and C++ examples against the wickra-synth C ABI.
#
#   cargo build --release -p wickra-synth-c
#   cmake -S examples/c -B examples/c/build
#   cmake --build examples/c/build --config Release
#   ctest --test-dir examples/c/build -C Release --output-on-failure
#
# WICKRA_SYNTH_LIB_DIR defaults to the workspace target/release directory and can
# be overridden for out-of-tree builds.

cmake_minimum_required(VERSION 3.15)
project(wickra_synth_examples C CXX)

set(CMAKE_C_STANDARD 11)
set(CMAKE_C_STANDARD_REQUIRED ON)
set(CMAKE_CXX_STANDARD 14)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

set(WICKRA_SYNTH_LIB_DIR "${CMAKE_CURRENT_SOURCE_DIR}/../../target/release"
    CACHE PATH "Directory containing the built wickra-synth C ABI library")

set(WICKRA_SYNTH_INCLUDE "${CMAKE_CURRENT_SOURCE_DIR}/../../bindings/c/include")

# Absolute path to the cross-language golden corpus, baked into the golden tests
# as a compile definition so they run from any working directory (ctest runs
# them from the build tree, not from the source tree).
get_filename_component(WICKRA_SYNTH_GOLDEN_DIR "${CMAKE_CURRENT_SOURCE_DIR}/../../golden" ABSOLUTE)

# The fixture names are derived from the corpus itself rather than listed here,
# so a new spec is picked up without editing C. A hand-kept list would go stale
# silently: the test would still pass, just over fewer fixtures.
file(GLOB WICKRA_SYNTH_GOLDEN_FILES RELATIVE "${WICKRA_SYNTH_GOLDEN_DIR}/specs"
     "${WICKRA_SYNTH_GOLDEN_DIR}/specs/*.json")
set(WICKRA_SYNTH_GOLDEN_NAMES "")
foreach(spec_file ${WICKRA_SYNTH_GOLDEN_FILES})
    get_filename_component(spec_name "${spec_file}" NAME_WE)
    string(APPEND WICKRA_SYNTH_GOLDEN_NAMES "\"${spec_name}\",")
endforeach()
if(WICKRA_SYNTH_GOLDEN_NAMES STREQUAL "")
    message(FATAL_ERROR "no golden fixtures found under ${WICKRA_SYNTH_GOLDEN_DIR}/specs")
endif()

find_library(WICKRA_SYNTH_LIB
    NAMES wickra_synth libwickra_synth
    HINTS "${WICKRA_SYNTH_LIB_DIR}"
    REQUIRED)

add_executable(gen gen.c)
add_executable(genpp gen.cpp)
add_executable(golden_test golden_test.c)
add_executable(golden_testpp golden_test.cpp)

set(WICKRA_SYNTH_EXAMPLES gen genpp golden_test golden_testpp)

foreach(example ${WICKRA_SYNTH_EXAMPLES})
    target_include_directories(${example} PRIVATE "${WICKRA_SYNTH_INCLUDE}")
    target_link_libraries(${example} PRIVATE "${WICKRA_SYNTH_LIB}")
endforeach()

foreach(example golden_test golden_testpp)
    target_compile_definitions(${example} PRIVATE
        "WICKRA_SYNTH_GOLDEN_DIR=\"${WICKRA_SYNTH_GOLDEN_DIR}\""
        "WICKRA_SYNTH_GOLDEN_NAMES=${WICKRA_SYNTH_GOLDEN_NAMES}")
endforeach()

# On Windows the runtime DLL must sit next to the executable (there is no rpath);
# without it the loader pops a modal "DLL not found" dialog that hangs ctest. Copy
# it next to each example after build. (Unix resolves the .so/.dylib via the
# linker's embedded path, so this is Windows-only.)
if(WIN32)
    foreach(example ${WICKRA_SYNTH_EXAMPLES})
        add_custom_command(TARGET ${example} POST_BUILD
            COMMAND ${CMAKE_COMMAND} -E copy_if_different
                "${WICKRA_SYNTH_LIB_DIR}/wickra_synth.dll"
                "$<TARGET_FILE_DIR:${example}>"
            VERBATIM)
    endforeach()
endif()

enable_testing()
foreach(example ${WICKRA_SYNTH_EXAMPLES})
    add_test(NAME ${example} COMMAND ${example})
endforeach()
# A missing runtime dependency must fail fast, never block the runner for hours.
set_tests_properties(${WICKRA_SYNTH_EXAMPLES} PROPERTIES TIMEOUT 60)
