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

file(GLOB GAME_C_FILES CONFIGURE_DEPENDS ${CMAKE_CURRENT_SOURCE_DIR}/src/game/*.c)
list(REMOVE_ITEM GAME_C_FILES ${CMAKE_CURRENT_SOURCE_DIR}/src/game/routine_0029.c)
list(REMOVE_ITEM GAME_C_FILES ${CMAKE_CURRENT_SOURCE_DIR}/src/game/routine_0104.c)
list(REMOVE_ITEM GAME_C_FILES ${CMAKE_CURRENT_SOURCE_DIR}/src/game/routine_0105.c)
list(REMOVE_ITEM GAME_C_FILES ${CMAKE_CURRENT_SOURCE_DIR}/src/game/routine_0176.c)
list(REMOVE_ITEM GAME_C_FILES ${CMAKE_CURRENT_SOURCE_DIR}/src/game/routine_0174.c)
list(REMOVE_ITEM GAME_C_FILES ${CMAKE_CURRENT_SOURCE_DIR}/src/game/routine_0177.c)
list(REMOVE_ITEM GAME_C_FILES ${CMAKE_CURRENT_SOURCE_DIR}/src/game/routine_0187.c)
list(REMOVE_ITEM GAME_C_FILES ${CMAKE_CURRENT_SOURCE_DIR}/src/game/routine_0189.c)
list(REMOVE_ITEM GAME_C_FILES ${CMAKE_CURRENT_SOURCE_DIR}/src/game/routine_0190.c)
list(REMOVE_ITEM GAME_C_FILES ${CMAKE_CURRENT_SOURCE_DIR}/src/game/routine_0191.c)
list(REMOVE_ITEM GAME_C_FILES ${CMAKE_CURRENT_SOURCE_DIR}/src/game/routine_0058.c)
list(REMOVE_ITEM GAME_C_FILES ${CMAKE_CURRENT_SOURCE_DIR}/src/game/queue_ppu_job_and_wait.c)
list(REMOVE_ITEM GAME_C_FILES ${CMAKE_CURRENT_SOURCE_DIR}/src/game/routine_0039.c)
list(REMOVE_ITEM GAME_C_FILES ${CMAKE_CURRENT_SOURCE_DIR}/src/game/routine_0002.c)
list(REMOVE_ITEM GAME_C_FILES ${CMAKE_CURRENT_SOURCE_DIR}/src/game/routine_0033.c)
list(REMOVE_ITEM GAME_C_FILES ${CMAKE_CURRENT_SOURCE_DIR}/src/game/routine_0034.c)
list(REMOVE_ITEM GAME_C_FILES ${CMAKE_CURRENT_SOURCE_DIR}/src/game/routine_0055.c)
list(REMOVE_ITEM GAME_C_FILES ${CMAKE_CURRENT_SOURCE_DIR}/src/game/routine_0045.c)
list(REMOVE_ITEM GAME_C_FILES ${CMAKE_CURRENT_SOURCE_DIR}/src/game/routine_0067.c)
list(REMOVE_ITEM GAME_C_FILES ${CMAKE_CURRENT_SOURCE_DIR}/src/game/routine_0148.c)
list(REMOVE_ITEM GAME_C_FILES ${CMAKE_CURRENT_SOURCE_DIR}/src/game/routine_0259.c)
list(REMOVE_ITEM GAME_C_FILES ${CMAKE_CURRENT_SOURCE_DIR}/src/game/main_loop_dispatch.c)
list(REMOVE_ITEM GAME_C_FILES ${CMAKE_CURRENT_SOURCE_DIR}/src/game/routine_0001.c)
list(REMOVE_ITEM GAME_C_FILES ${CMAKE_CURRENT_SOURCE_DIR}/src/game/routine_0049.c)
list(REMOVE_ITEM GAME_C_FILES ${CMAKE_CURRENT_SOURCE_DIR}/src/game/routine_0175.c)
list(REMOVE_ITEM GAME_C_FILES ${CMAKE_CURRENT_SOURCE_DIR}/src/game/routine_0020.c)
list(REMOVE_ITEM GAME_C_FILES ${CMAKE_CURRENT_SOURCE_DIR}/src/game/routine_0004.c)
list(REMOVE_ITEM GAME_C_FILES ${CMAKE_CURRENT_SOURCE_DIR}/src/game/routine_0050.c)
list(REMOVE_ITEM GAME_C_FILES ${CMAKE_CURRENT_SOURCE_DIR}/src/game/routine_0070.c)
list(REMOVE_ITEM GAME_C_FILES ${CMAKE_CURRENT_SOURCE_DIR}/src/game/routine_0071.c)
list(REMOVE_ITEM GAME_C_FILES ${CMAKE_CURRENT_SOURCE_DIR}/src/game/routine_0072.c)
list(REMOVE_ITEM GAME_C_FILES ${CMAKE_CURRENT_SOURCE_DIR}/src/game/routine_0068.c)
list(REMOVE_ITEM GAME_C_FILES ${CMAKE_CURRENT_SOURCE_DIR}/src/game/routine_0069.c)
list(REMOVE_ITEM GAME_C_FILES ${CMAKE_CURRENT_SOURCE_DIR}/src/game/routine_0074.c)
list(REMOVE_ITEM GAME_C_FILES ${CMAKE_CURRENT_SOURCE_DIR}/src/game/routine_0133.c)
list(REMOVE_ITEM GAME_C_FILES ${CMAKE_CURRENT_SOURCE_DIR}/src/game/routine_0134.c)
add_library(lotw_native_objects OBJECT
  src/native/game_state.cc
  src/native/frame_runner.cc
  src/native/lotw_scripts.cc
  src/native/queue_ppu_job_and_wait.cc
  src/native/main_loop_dispatch.cc
  src/native/routine_0058.cc
  src/native/routine_0039.cc
  src/native/routine_0002.cc
  src/native/routine_0033.cc
  src/native/routine_0034.cc
  src/native/routine_0055.cc
  src/native/routine_0001.cc
  src/native/routine_0049.cc
  src/native/routine_0175.cc
  src/native/routine_0045.cc
  src/native/routine_0067.cc
  src/native/routine_0148.cc
  src/native/routine_0259.cc
  src/native/routine_0029.cc
  src/native/routine_0104.cc
  src/native/routine_0105.cc
  src/native/routine_0174.cc
  src/native/routine_0177.cc
  src/native/routine_0176.cc
  src/native/routine_0187.cc
  src/native/routine_0189.cc
  src/native/routine_0190.cc
  src/native/routine_0191.cc
  src/native/routine_0020.cc
  src/native/routine_0004.cc
  src/native/routine_0050.cc
  src/native/routine_0070.cc
  src/native/routine_0071.cc
  src/native/routine_0072.cc
  src/native/routine_0068.cc
  src/native/routine_0069.cc
  src/native/routine_0074.cc
  src/native/routine_0133.cc
  src/native/routine_0134.cc)
target_include_directories(lotw_native_objects PUBLIC ${CMAKE_CURRENT_SOURCE_DIR}/src)
target_compile_definitions(lotw_native_objects PUBLIC LOTW_HOST LOTW_SHIM)
add_library(lotw_engine STATIC ${GAME_C_FILES} src/ppu.c src/apu.c $<TARGET_OBJECTS:lotw_native_objects>)
target_include_directories(lotw_engine PUBLIC ${CMAKE_CURRENT_SOURCE_DIR}/src)
target_compile_definitions(lotw_engine PUBLIC LOTW_HOST LOTW_SHIM)
target_link_libraries(lotw_engine PUBLIC m Threads::Threads)

find_package(PkgConfig REQUIRED)
pkg_check_modules(SDL3 REQUIRED IMPORTED_TARGET sdl3)
add_executable(play test/play_sdl.cc)
target_link_libraries(play PRIVATE lotw_engine PkgConfig::SDL3)

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
add_test(NAME port_contract
  COMMAND ${Python3_EXECUTABLE} ${CMAKE_CURRENT_SOURCE_DIR}/tools/verify/check_port_contract.py)
