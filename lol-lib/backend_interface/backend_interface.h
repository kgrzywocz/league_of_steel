#pragma once

#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <new>

struct LolStats {
  uint8_t health;
  uint8_t mana;
  uint8_t hit;
};

struct BackendScreenAnalyzer {
  uint8_t phantom_data;
};

struct BackendPixelRect {
  uint8_t phantom_data;
};

using FrontendAnalysisFunction = LolStats(*)(const BackendPixelRect*);

struct BackendScreenResolution {
  uint32_t width;
  uint32_t height;
};

struct BackendColor {
  uint8_t b;
  uint8_t g;
  uint8_t r;
  uint8_t a;
};

struct BackendCaptureRect {
  int32_t left;
  int32_t top;
  int32_t right;
  int32_t bottom;
};

extern "C" {

extern LolStats lollib_backend_analyzeScreenshot(BackendScreenAnalyzer *s);

extern BackendScreenAnalyzer *lollib_backend_createBackendScreenAnalyzer(FrontendAnalysisFunction analyzeFunction);

extern void lollib_backend_destroyBackendScreenAnalyzer(BackendScreenAnalyzer *s);

extern BackendScreenResolution lollib_backend_getMode(BackendScreenAnalyzer *s);

extern BackendColor lollib_backend_pixelRect_getColor(const BackendPixelRect *rect,
                                                      int32_t row,
                                                      int32_t column);

extern int32_t lollib_backend_pixelRect_getHight(const BackendPixelRect *rect);

extern int32_t lollib_backend_pixelRect_getWidth(const BackendPixelRect *rect);

extern void lollib_backend_setCaptureRect(BackendScreenAnalyzer *s,
                                          const BackendCaptureRect *captureRect);

} // extern "C"
