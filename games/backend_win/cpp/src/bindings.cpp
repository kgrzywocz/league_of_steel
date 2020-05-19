#include "bindings.h"

#include "ProcessHelper.hpp"
#include "callSafely.hpp"

extern "C" int32_t lollib_is_process_running(const char exe_name[])
{
  return callSafely<int32_t>([=]() { return isProcessRunning(exe_name); });
}

extern "C" void lollib_get_process_exe_path(const char exe_name[], char *output, size_t output_length)
{
  auto path = callSafely<std::string>([=]() { return getProcessRunningPath(exe_name); });
  path.copy(output, output_length);
}