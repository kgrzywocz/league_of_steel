#pragma once

#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <new>

struct AnalyzerHolder;

struct BackendScreenAnalyzer {
  uint8_t phantom_data;
};

struct BackendPixelRect {
  uint8_t phantom_data;
};

using FrontendAnalysisFunction = void(*)(AnalyzerHolder*, const BackendPixelRect*);

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

extern "C" {

extern void lollib_backend_analyzeScreenshot(BackendScreenAnalyzer *s,
                                             AnalyzerHolder *analyzer_holder,
                                             FrontendAnalysisFunction analyzeFunction);

extern BackendScreenAnalyzer *lollib_backend_createBackendScreenAnalyzer();

extern void lollib_backend_destroyBackendScreenAnalyzer(BackendScreenAnalyzer *s);

extern BackendScreenResolution lollib_backend_getMode(BackendScreenAnalyzer *s);

extern void lollib_backend_get_process_exe_path(const char *exe_name,
                                                char *output,
                                                size_t output_length);

extern int32_t lollib_backend_is_process_running(const char *exe_name);

extern BackendColor lollib_backend_pixelRect_getColor(const BackendPixelRect *rect,
                                                      uint32_t x,
                                                      uint32_t y);

extern uint32_t lollib_backend_pixelRect_getHight(const BackendPixelRect *rect);

extern uint32_t lollib_backend_pixelRect_getWidth(const BackendPixelRect *rect);

} // extern "C"
