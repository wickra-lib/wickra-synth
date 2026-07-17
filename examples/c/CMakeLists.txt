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

find_library(WICKRA_SYNTH_LIB
    NAMES wickra_synth libwickra_synth
    HINTS "${WICKRA_SYNTH_LIB_DIR}"
    REQUIRED)

add_executable(gen gen.c)
target_include_directories(gen PRIVATE "${WICKRA_SYNTH_INCLUDE}")
target_link_libraries(gen PRIVATE "${WICKRA_SYNTH_LIB}")

add_executable(genpp gen.cpp)
target_include_directories(genpp PRIVATE "${WICKRA_SYNTH_INCLUDE}")
target_link_libraries(genpp PRIVATE "${WICKRA_SYNTH_LIB}")

# On Windows the runtime DLL must sit next to the executable (there is no rpath);
# without it the loader pops a modal "DLL not found" dialog that hangs ctest. Copy
# it next to each example after build. (Unix resolves the .so/.dylib via the
# linker's embedded path, so this is Windows-only.)
if(WIN32)
    foreach(example gen genpp)
        add_custom_command(TARGET ${example} POST_BUILD
            COMMAND ${CMAKE_COMMAND} -E copy_if_different
                "${WICKRA_SYNTH_LIB_DIR}/wickra_synth.dll"
                "$<TARGET_FILE_DIR:${example}>"
            VERBATIM)
    endforeach()
endif()

enable_testing()
add_test(NAME gen COMMAND gen)
add_test(NAME genpp COMMAND genpp)
# A missing runtime dependency must fail fast, never block the runner for hours.
set_tests_properties(gen genpp PROPERTIES TIMEOUT 60)
