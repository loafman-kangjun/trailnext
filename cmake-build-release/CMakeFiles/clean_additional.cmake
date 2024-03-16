# Additional clean files
cmake_minimum_required(VERSION 3.16)

if("${CONFIG}" STREQUAL "" OR "${CONFIG}" STREQUAL "Release")
  file(REMOVE_RECURSE
  "CMakeFiles\\appTrailNext_autogen.dir\\AutogenUsed.txt"
  "CMakeFiles\\appTrailNext_autogen.dir\\ParseCache.txt"
  "FluentUI\\src\\CMakeFiles\\fluentuiplugin_autogen.dir\\AutogenUsed.txt"
  "FluentUI\\src\\CMakeFiles\\fluentuiplugin_autogen.dir\\ParseCache.txt"
  "FluentUI\\src\\fluentuiplugin_autogen"
  "appTrailNext_autogen"
  )
endif()
