#include "bindings.h"

#include "ProcessHelper.hpp"
#include "callSafely.hpp"

extern "C" int32_t lollib_is_lol_running()
{
  return callSafely<int32_t>([]() { return isProcessRunning("League of Legends.exe"); });
}

extern "C" void lollib_lol_exe_path(char *output, size_t output_length)
{
  auto path = callSafely<std::string>([]() { return getProcessRunningPath("League of Legends.exe"); });
  path.copy(output, output_length);
}