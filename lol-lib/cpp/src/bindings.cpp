#include "bindings.h"

#include "LolLib.hpp"

static LolLib *lollib;

#include "callSafely.hpp"

extern "C" int32_t lollib_init()
{
  callSafely<int32_t>([]() {
    if (lollib)
    {
      delete lollib;
      lollib = nullptr;
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
      lollib = nullptr;
    }
    return 0;
  });
}

extern "C" int32_t lollib_screen_width()
{
  return callSafely_member<int32_t>(lollib, []() { return lollib->getScreenWidth(); });
}
extern "C" int32_t lollib_screen_height()
{
  return callSafely_member<int32_t>(lollib, []() { return lollib->getScreenHeight(); });
}

extern "C" int32_t lollib_has_mode_changed()
{
  return callSafely_member<bool>(lollib, []() { return lollib->hasModeChanged(); });
}

extern "C" LolStats lollib_get_stats()
{
  return callSafely_member<LolStats>(lollib, []() { return lollib->getCurrentStats(); });
}

extern "C" void lollib_set_hud_scaling(float hudGlobalScale)
{
  callSafely_member<bool>(lollib, [=]() { lollib->setHudScaling(hudGlobalScale); return true; });
}
