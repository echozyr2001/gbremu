file(GLOB sources CONFIGURE_DEPENDS ${PROJECT_SOURCE_DIR}/lib/*.c)

add_library(emu STATIC ${sources})

target_include_directories(emu PUBLIC ${PROJECT_SOURCE_DIR}/include)

find_package(SDL2 REQUIRED)
find_package(SDL2_ttf REQUIRED)

target_include_directories(emu PUBLIC ${SDL2_INCLUDE_DIRS})
target_link_libraries(emu PUBLIC SDL2::SDL2 SDL2_ttf::SDL2_ttf)