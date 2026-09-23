cmake_minimum_required(VERSION 3.11)
project(exercise-4)

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

find_package(Z3 REQUIRED CONFIG)

add_executable(exercise-4 src/main.cpp)
target_link_libraries(exercise-4 z3::libz3)

install(TARGETS exercise-4)
