#pragma once

#include <stddef.h>
#include <stdint.h>

extern "C" int32_t lollib_is_lol_running();
extern "C" void lollib_lol_exe_path(char *output, size_t output_length);