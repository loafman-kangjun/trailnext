cmake_minimum_required(VERSION 3.20)
project(FluentUI VERSION 1.0)

list(APPEND CMAKE_MODULE_PATH ${CMAKE_SOURCE_DIR}/.cmake/)

option(FLUENTUI_BUILD_EXAMPLES "Build FluentUI demo applications." OFF)

find_package(QT NAMES Qt6 Qt5 REQUIRED COMPONENTS Core)
find_package(Qt${QT_VERSION_MAJOR} REQUIRED COMPONENTS Core)

add_subdirectory(src)

target_compile_definitions(fluentuiplugin
    PRIVATE
    QT_MESSAGELOGCONTEXT
)

