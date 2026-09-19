cmake_minimum_required(VERSION 3.11)
project(z3_assignment)

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

find_package(Z3 REQUIRED CONFIG)

add_executable(main main.cpp)
target_link_libraries(main z3::libz3)
