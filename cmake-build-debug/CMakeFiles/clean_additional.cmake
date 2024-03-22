# Additional clean files
cmake_minimum_required(VERSION 3.16)

if("${CONFIG}" STREQUAL "" OR "${CONFIG}" STREQUAL "Debug")
  file(REMOVE_RECURSE
  "CMakeFiles\\appTrailNext_autogen.dir\\AutogenUsed.txt"
  "CMakeFiles\\appTrailNext_autogen.dir\\ParseCache.txt"
  "FluentUI\\src\\CMakeFiles\\fluentuiplugin_autogen.dir\\AutogenUsed.txt"
  "FluentUI\\src\\CMakeFiles\\fluentuiplugin_autogen.dir\\ParseCache.txt"
  "FluentUI\\src\\fluentuiplugin_autogen"
  "appTrailNext_autogen"
  "util\\CMakeFiles\\Util_autogen.dir\\AutogenUsed.txt"
  "util\\CMakeFiles\\Util_autogen.dir\\ParseCache.txt"
  "util\\Util_autogen"
  )
endif()
