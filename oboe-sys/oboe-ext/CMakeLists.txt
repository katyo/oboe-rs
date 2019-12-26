cmake_minimum_required(VERSION 3.4.1)

project(oboe-ext)

set(oboe-ext_sources
    src/AudioStreamCallbackWrapper.cpp
    )

add_library (oboe-ext ${oboe-ext_sources})

#set (OBOE_DIR ../oboe)

target_include_directories(oboe-ext
    PRIVATE src
    PUBLIC include ${OBOE_DIR}/include)

target_compile_options(oboe-ext
    PRIVATE
    -std=c++14
    -Wall
    -Wextra-semi
    -Wshadow
    -Wshadow-field
    -Ofast
    "$<$<CONFIG:DEBUG>:-Werror>")

add_subdirectory (${OBOE_DIR} ./oboe)

target_link_libraries (oboe-ext oboe)
