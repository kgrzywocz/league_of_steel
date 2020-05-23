#include "backend_interface.h"

#include "ScreenAnalyzer.hpp"
#include "PixelRect.hpp"
#include "callSafely.hpp"

extern "C" BackendScreenAnalyzer *lollib_backend_createBackendScreenAnalyzer()
{
    return callSafely<BackendScreenAnalyzer *>([=]() {
        return reinterpret_cast<BackendScreenAnalyzer *>(
            new ScreenAnalyzer());
    });
}
extern "C" void lollib_backend_destroyBackendScreenAnalyzer(BackendScreenAnalyzer *screenAnalyzer)
{
    callSafely_member<int32_t>(screenAnalyzer, [=]() {
        delete reinterpret_cast<ScreenAnalyzer *>(screenAnalyzer);
        return 0;
    });
}
extern "C" BackendScreenResolution lollib_backend_getMode(BackendScreenAnalyzer *screenAnalyzer)
{
    return callSafely_member<BackendScreenResolution>(screenAnalyzer, [=]() {
        return reinterpret_cast<ScreenAnalyzer *>(screenAnalyzer)->getMode();
    });
}
extern "C" void lollib_backend_analyzeScreenshot(BackendScreenAnalyzer *screenAnalyzer,
                                                 AnalyzerHolder *analyzer_holder,
                                                 FrontendAnalysisFunction analyzeFunction)
{
    callSafely_member<int32_t>(screenAnalyzer, [=]() {
        reinterpret_cast<ScreenAnalyzer *>(screenAnalyzer)->analyzeScreenshot([=](const PixelRect &rect) {
            analyzeFunction(analyzer_holder, reinterpret_cast<const BackendPixelRect *>(&rect));
        });
        return 0;
    });
}

extern "C" uint32_t lollib_backend_pixelRect_getHight(const BackendPixelRect *rect)
{
    return callSafely_member<uint32_t>(rect, [=]() {
        return reinterpret_cast<const PixelRect *>(rect)->getHight();
    });
}
extern "C" uint32_t lollib_backend_pixelRect_getWidth(const BackendPixelRect *rect)
{
    return callSafely_member<uint32_t>(rect, [=]() {
        return reinterpret_cast<const PixelRect *>(rect)->getWidth();
    });
}
extern "C" BackendColor lollib_backend_pixelRect_getColor(const BackendPixelRect *rect, uint32_t x, uint32_t y)
{
    return callSafely_member<BackendColor>(rect, [=]() {
        return reinterpret_cast<const PixelRect *>(rect)->getColor(x, y);
    });
}