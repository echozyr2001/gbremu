set(TEST_SOURCES check_gbe.c)

find_path(CHECK_INCLUIDE_DIR check.h)
find_library(CHECK_LIBRARIES check)

add_executable(check_gbe ${TEST_SOURCES})

target_include_directories(check_gbe PRIVATE ${PROJECT_SOURCE_DIR} ${CHECK_INCLUIDE_DIR})
target_link_libraries(check_gbe emu ${CHECK_LIBRARIES})