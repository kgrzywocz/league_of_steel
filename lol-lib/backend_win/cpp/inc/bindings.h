#pragma once

#include <stddef.h>
#include <stdint.h>

extern "C" int32_t lollib_is_process_running(const char exe_name[]);
extern "C" void lollib_get_process_exe_path(const char exe_name[], char *output, size_t output_length);