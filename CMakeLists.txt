# Legacy of the Wizard — C port build.
#
# Builds the decompiled engine (321 ported 6502 routines + software PPU/APU
# shims) as a static library, then the SDL playable front-end and the headless
# host test drivers on top of it.
#
#   nix develop --command cmake -S . -B build
#   nix develop --command cmake --build build -j
#   ./build/play rom/lotw.nes          # window + audio + keyboard/gamepad
#
# The matching-disassembly diff-test harness (test/bulk_harness.c) is built by
# the Python RE tooling, not here — it needs the non-shim build + generated glue.
cmake_minimum_required(VERSION 3.20)
project(lotw_port C)

set(CMAKE_C_STANDARD 11)
set(CMAKE_C_STANDARD_REQUIRED ON)
if(NOT CMAKE_BUILD_TYPE)
  set(CMAKE_BUILD_TYPE Release CACHE STRING "" FORCE)
endif()
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)
find_package(Python3 COMPONENTS Interpreter REQUIRED)

# --- engine: ported routines + PPU/APU shims -------------------------------
file(GLOB PORTED CONFIGURE_DEPENDS ${CMAKE_CURRENT_SOURCE_DIR}/src/ported/*.c)
add_library(lotw_engine STATIC ${PORTED} src/ppu.c src/apu.c)
target_include_directories(lotw_engine PUBLIC ${CMAKE_CURRENT_SOURCE_DIR}/src)
# LOTW_HOST: 64 KiB flat address space. LOTW_SHIM: live PPU/APU/controller paths.
target_compile_definitions(lotw_engine PUBLIC LOTW_HOST LOTW_SHIM)
target_link_libraries(lotw_engine PUBLIC m)

# --- SDL3 playable front-end -----------------------------------------------
find_package(PkgConfig REQUIRED)
pkg_check_modules(SDL3 REQUIRED IMPORTED_TARGET sdl3)
add_executable(play test/play_sdl.c)
target_link_libraries(play PRIVATE lotw_engine PkgConfig::SDL3)

# --- headless host test drivers (no SDL) -----------------------------------
foreach(drv apu_test ppu_anim ppu_shim_test ppu_game_driver lockstep_port replay_capture_port item_pickup_test)
  add_executable(${drv} test/${drv}.c)
  target_link_libraries(${drv} PRIVATE lotw_engine)
endforeach()

enable_testing()
add_test(NAME item_pickup_test COMMAND item_pickup_test)
add_test(NAME scheduler_contract
  COMMAND ${Python3_EXECUTABLE} ${CMAKE_CURRENT_SOURCE_DIR}/tools/re/check_scheduler_contract.py)
