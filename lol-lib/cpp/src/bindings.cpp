#include "bindings.h"

#include "LolLib.hpp"

#include "ProcessHelper.hpp"

static LolLib *lollib;

#include "callSafely.hpp"

extern "C" int32_t lollib_init()
{
  callSafely<int32_t>([]() {
    if (lollib)
    {
      delete lollib;
    }
    lollib = new LolLib;
    return 0;
  });

  return 0;
}
extern "C" void lollib_destroy()
{
  callSafely<int32_t>([]() {
    if (lollib)
    {
      delete lollib;
    }
    return 0;
  });
}

extern "C" int32_t lollib_screen_width()
{
  return callSafely_member<int32_t>([]() { return lollib->getMode().Width; });
}
extern "C" int32_t lollib_screen_height()
{
  return callSafely_member<int32_t>([]() { return lollib->getMode().Height; });
}

extern "C" int32_t lollib_has_mode_changed()
{
  return callSafely_member<bool>([]() { return lollib->hasModeChanged(); });
}

extern "C" LolStats lollib_get_stats() //ujednolicenie nazw funkcji
{
  return callSafely_member<LolStats>([]() { return lollib->getCurrentStats(); });
}

extern "C" int32_t lollib_is_lol_running()
{
  return callSafely<int32_t>([]() { return isProcessRunning("League of Legends.exe"); });
}