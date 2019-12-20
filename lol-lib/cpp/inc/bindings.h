#pragma once

#include "LolStats.h"

extern "C" int32_t lollib_init();
extern "C" void lollib_destroy();
extern "C" int32_t lollib_screen_width();
extern "C" int32_t lollib_screen_height();
extern "C" int32_t lollib_has_mode_changed();
extern "C" LolStats lollib_get_stats();
extern "C" int32_t lollib_is_lol_running();