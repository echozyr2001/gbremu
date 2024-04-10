cmake_minimum_required(VERSION 3.5...3.21 FATAL_ERROR)

project(gbemu C)

add_subdirectory(lib)
add_subdirectory(gbemu)
add_subdirectory(tests)