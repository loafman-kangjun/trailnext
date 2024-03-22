cmake_minimum_required(VERSION 3.24)

project(Util VERSION 0.1)

find_package(cmark CONFIG REQUIRED)
find_package(glog CONFIG REQUIRED)
find_package(gRPC CONFIG REQUIRED)

add_executable(Util
        markdown/markdown.cpp
        markdown/markdown.h
)

target_link_libraries(Util
        glog::glog cmark::cmark
)