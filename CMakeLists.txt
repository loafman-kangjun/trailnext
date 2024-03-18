cmake_minimum_required(VERSION 3.16)

project(TrailNext VERSION 0.1 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(FLUENTUI_BUILD_EXAMPLES OFF)

find_package (glog 0.7.0 REQUIRED)
find_package(Qt6 6.5 REQUIRED COMPONENTS Quick LinguistTools)

qt_standard_project_setup(REQUIRES 6.5)

qt_add_executable(appTrailNext
    main.cpp
)

qt_add_qml_module(appTrailNext
    URI TrailNext
    VERSION 1.0
    QML_FILES Main.qml
)

set_target_properties(appTrailNext PROPERTIES
#    MACOSX_BUNDLE_GUI_IDENTIFIER com.example.appTrailNext
    MACOSX_BUNDLE_BUNDLE_VERSION ${PROJECT_VERSION}
    MACOSX_BUNDLE_SHORT_VERSION_STRING ${PROJECT_VERSION_MAJOR}.${PROJECT_VERSION_MINOR}
    MACOSX_BUNDLE TRUE
    WIN32_EXECUTABLE TRUE
)

target_link_libraries(appTrailNext
    PRIVATE Qt6::Quick glog::glog
)

include(GNUInstallDirs)
install(TARGETS appTrailNext
    BUNDLE DESTINATION .
    LIBRARY DESTINATION ${CMAKE_INSTALL_LIBDIR}
    RUNTIME DESTINATION ${CMAKE_INSTALL_BINDIR}
)

qt_add_translations(appTrailNext TS_FILES ./translate/TrailNext_zh_CN.ts)

add_subdirectory(FluentUI)
