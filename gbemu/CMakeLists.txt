set(MAIN_SOUECES main.c)

add_executable(gbemu ${MAIN_SOUECES})

target_include_directories(gbemu PRIVATE emu)
target_link_libraries(gbemu emu)