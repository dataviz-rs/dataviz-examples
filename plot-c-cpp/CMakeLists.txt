# dataviz from C and C++: CMake fetches the dataviz repository (tag
# c-v0.19.1) and builds its C library with cargo.
#
#   cmake -S . -B build -DCMAKE_BUILD_TYPE=Release
#   cmake --build build --config Release
#   ./build/dashboard        (Windows: build\Release\dashboard.exe)
#
# A local checkout instead of the download:
#   cmake -S . -B build -DFETCHCONTENT_SOURCE_DIR_DATAVIZ=../../dataviz
cmake_minimum_required(VERSION 3.16)
project(dataviz_c_cpp_example LANGUAGES C CXX)

include(FetchContent)
FetchContent_Declare(dataviz
  GIT_REPOSITORY https://github.com/dataviz-rs/dataviz.git
  GIT_TAG c-v0.19.1
  GIT_SHALLOW TRUE
  SOURCE_SUBDIR capi)
FetchContent_MakeAvailable(dataviz)

add_executable(dashboard dashboard.cpp)
target_link_libraries(dashboard PRIVATE dataviz::dataviz)
set_target_properties(dashboard PROPERTIES CXX_STANDARD 17)

add_executable(live live.c)
target_link_libraries(live PRIVATE dataviz::dataviz)
if(NOT WIN32)
  target_link_libraries(live PRIVATE m)
endif()

if(WIN32)
  foreach(t dashboard live)
    add_custom_command(TARGET ${t} POST_BUILD
      COMMAND ${CMAKE_COMMAND} -E copy_if_different ${DATAVIZ_SHARED_LIBRARY} $<TARGET_FILE_DIR:${t}>)
  endforeach()
endif()
