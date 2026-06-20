# Legacy of the Wizard — C port build.
#
# Builds the ported engine (legacy C routines + software PPU/APU shims) as a
# static library, then the SDL playable front-end and the headless host test
# drivers on top of it.
#
#   nix develop --command cmake -S . -B build
#   nix develop --command cmake --build build -j
#   ./build/play rom/lotw.nes          # window + audio + keyboard/gamepad
#
# Older reverse-engineering harnesses are kept out of this build; this CMake
# path is the native playable/testable port.
cmake_minimum_required(VERSION 3.20)
project(lotw_port C CXX)

set(CMAKE_C_STANDARD 11)
set(CMAKE_C_STANDARD_REQUIRED ON)
set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
if(NOT CMAKE_BUILD_TYPE)
  set(CMAKE_BUILD_TYPE Release CACHE STRING "" FORCE)
endif()
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)
find_package(Python3 COMPONENTS Interpreter REQUIRED)
find_package(Threads REQUIRED)

# --- engine: ported routines + native coroutine replacements + PPU/APU shims -
file(GLOB PORTED CONFIGURE_DEPENDS ${CMAKE_CURRENT_SOURCE_DIR}/src/ported/*.c)
list(REMOVE_ITEM PORTED ${CMAKE_CURRENT_SOURCE_DIR}/src/ported/sub_AE11.c)
list(REMOVE_ITEM PORTED ${CMAKE_CURRENT_SOURCE_DIR}/src/ported/sub_CC17.c)
list(REMOVE_ITEM PORTED ${CMAKE_CURRENT_SOURCE_DIR}/src/ported/sub_CC2D.c)
list(REMOVE_ITEM PORTED ${CMAKE_CURRENT_SOURCE_DIR}/src/ported/sub_E27D.c)
list(REMOVE_ITEM PORTED ${CMAKE_CURRENT_SOURCE_DIR}/src/ported/sub_E00F.c)
list(REMOVE_ITEM PORTED ${CMAKE_CURRENT_SOURCE_DIR}/src/ported/sub_E2AA.c)
list(REMOVE_ITEM PORTED ${CMAKE_CURRENT_SOURCE_DIR}/src/ported/sub_E424.c)
list(REMOVE_ITEM PORTED ${CMAKE_CURRENT_SOURCE_DIR}/src/ported/sub_E514.c)
list(REMOVE_ITEM PORTED ${CMAKE_CURRENT_SOURCE_DIR}/src/ported/sub_E562.c)
list(REMOVE_ITEM PORTED ${CMAKE_CURRENT_SOURCE_DIR}/src/ported/sub_E5B4.c)
list(REMOVE_ITEM PORTED ${CMAKE_CURRENT_SOURCE_DIR}/src/ported/sub_C135.c)
list(REMOVE_ITEM PORTED ${CMAKE_CURRENT_SOURCE_DIR}/src/ported/queue_ppu_job_and_wait.c)
list(REMOVE_ITEM PORTED ${CMAKE_CURRENT_SOURCE_DIR}/src/ported/sub_B13D.c)
list(REMOVE_ITEM PORTED ${CMAKE_CURRENT_SOURCE_DIR}/src/ported/sub_A3E3.c)
list(REMOVE_ITEM PORTED ${CMAKE_CURRENT_SOURCE_DIR}/src/ported/sub_AE64.c)
list(REMOVE_ITEM PORTED ${CMAKE_CURRENT_SOURCE_DIR}/src/ported/sub_B0B1.c)
list(REMOVE_ITEM PORTED ${CMAKE_CURRENT_SOURCE_DIR}/src/ported/sub_B6A6.c)
list(REMOVE_ITEM PORTED ${CMAKE_CURRENT_SOURCE_DIR}/src/ported/sub_B29B.c)
list(REMOVE_ITEM PORTED ${CMAKE_CURRENT_SOURCE_DIR}/src/ported/sub_C3E5.c)
list(REMOVE_ITEM PORTED ${CMAKE_CURRENT_SOURCE_DIR}/src/ported/sub_DA31.c)
list(REMOVE_ITEM PORTED ${CMAKE_CURRENT_SOURCE_DIR}/src/ported/sub_F430.c)
list(REMOVE_ITEM PORTED ${CMAKE_CURRENT_SOURCE_DIR}/src/ported/main_loop_dispatch.c)
list(REMOVE_ITEM PORTED ${CMAKE_CURRENT_SOURCE_DIR}/src/ported/sub_A2EB.c)
list(REMOVE_ITEM PORTED ${CMAKE_CURRENT_SOURCE_DIR}/src/ported/sub_B307.c)
list(REMOVE_ITEM PORTED ${CMAKE_CURRENT_SOURCE_DIR}/src/ported/sub_E077.c)
list(REMOVE_ITEM PORTED ${CMAKE_CURRENT_SOURCE_DIR}/src/ported/sub_AAEE.c)
list(REMOVE_ITEM PORTED ${CMAKE_CURRENT_SOURCE_DIR}/src/ported/sub_A5B5.c)
list(REMOVE_ITEM PORTED ${CMAKE_CURRENT_SOURCE_DIR}/src/ported/sub_B4C5.c)
list(REMOVE_ITEM PORTED ${CMAKE_CURRENT_SOURCE_DIR}/src/ported/sub_C492.c)
list(REMOVE_ITEM PORTED ${CMAKE_CURRENT_SOURCE_DIR}/src/ported/sub_C4B4.c)
list(REMOVE_ITEM PORTED ${CMAKE_CURRENT_SOURCE_DIR}/src/ported/sub_C4E0.c)
list(REMOVE_ITEM PORTED ${CMAKE_CURRENT_SOURCE_DIR}/src/ported/sub_C430.c)
list(REMOVE_ITEM PORTED ${CMAKE_CURRENT_SOURCE_DIR}/src/ported/sub_C461.c)
list(REMOVE_ITEM PORTED ${CMAKE_CURRENT_SOURCE_DIR}/src/ported/sub_C540.c)
list(REMOVE_ITEM PORTED ${CMAKE_CURRENT_SOURCE_DIR}/src/ported/sub_D16A.c)
list(REMOVE_ITEM PORTED ${CMAKE_CURRENT_SOURCE_DIR}/src/ported/sub_D199.c)
add_library(lotw_native_objects OBJECT
  src/native/game_state.cc
  src/native/frame_runner.cc
  src/native/lotw_scripts.cc
  src/native/queue_ppu_job_and_wait.cc
  src/native/main_loop_dispatch.cc
  src/native/sub_C135.cc
  src/native/sub_B13D.cc
  src/native/sub_A3E3.cc
  src/native/sub_AE64.cc
  src/native/sub_B0B1.cc
  src/native/sub_B6A6.cc
  src/native/sub_A2EB.cc
  src/native/sub_B307.cc
  src/native/sub_E077.cc
  src/native/sub_B29B.cc
  src/native/sub_C3E5.cc
  src/native/sub_DA31.cc
  src/native/sub_F430.cc
  src/native/sub_AE11.cc
  src/native/sub_CC17.cc
  src/native/sub_CC2D.cc
  src/native/sub_E00F.cc
  src/native/sub_E2AA.cc
  src/native/sub_E27D.cc
  src/native/sub_E424.cc
  src/native/sub_E514.cc
  src/native/sub_E562.cc
  src/native/sub_E5B4.cc
  src/native/sub_AAEE.cc
  src/native/sub_A5B5.cc
  src/native/sub_B4C5.cc
  src/native/sub_C492.cc
  src/native/sub_C4B4.cc
  src/native/sub_C4E0.cc
  src/native/sub_C430.cc
  src/native/sub_C461.cc
  src/native/sub_C540.cc
  src/native/sub_D16A.cc
  src/native/sub_D199.cc)
target_include_directories(lotw_native_objects PUBLIC ${CMAKE_CURRENT_SOURCE_DIR}/src)
target_compile_definitions(lotw_native_objects PUBLIC LOTW_HOST LOTW_SHIM)
add_library(lotw_engine STATIC ${PORTED} src/ppu.c src/apu.c $<TARGET_OBJECTS:lotw_native_objects>)
target_include_directories(lotw_engine PUBLIC ${CMAKE_CURRENT_SOURCE_DIR}/src)
# LOTW_HOST: 64 KiB flat address space. LOTW_SHIM: live PPU/APU/controller paths.
target_compile_definitions(lotw_engine PUBLIC LOTW_HOST LOTW_SHIM)
target_link_libraries(lotw_engine PUBLIC m Threads::Threads)

# --- SDL3 playable front-end -----------------------------------------------
find_package(PkgConfig REQUIRED)
pkg_check_modules(SDL3 REQUIRED IMPORTED_TARGET sdl3)
add_executable(play test/play_sdl.cc)
target_link_libraries(play PRIVATE lotw_engine PkgConfig::SDL3)

# --- headless host test drivers (no SDL) -----------------------------------
foreach(drv apu_test ppu_anim ppu_shim_test ppu_game_driver lockstep_port replay_capture_port item_pickup_test)
  add_executable(${drv} test/${drv}.c)
  target_link_libraries(${drv} PRIVATE lotw_engine)
  set_property(TARGET ${drv} PROPERTY LINKER_LANGUAGE CXX)
endforeach()
add_executable(native_coroutine_test test/native_coroutine_test.cc)
target_link_libraries(native_coroutine_test PRIVATE lotw_engine)

enable_testing()
add_test(NAME item_pickup_test COMMAND item_pickup_test)
add_test(NAME native_coroutine_test COMMAND native_coroutine_test)
add_test(NAME scheduler_contract
  COMMAND ${Python3_EXECUTABLE} ${CMAKE_CURRENT_SOURCE_DIR}/tools/re/check_scheduler_contract.py)
