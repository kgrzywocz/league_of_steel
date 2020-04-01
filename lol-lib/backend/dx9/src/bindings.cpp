#include "backend_interface.h"

#include "ScreenAnalyzer.hpp"
#include "PixelRect.hpp"


extern "C" BackendScreenAnalyzer* lollib_backend_createBackendScreenAnalyzer(FrontendAnalysisFunction analyzeFunction)
{
    return reinterpret_cast<BackendScreenAnalyzer*>(
        new ScreenAnalyzer(
            [=](const PixelRect & rect){
                return analyzeFunction(reinterpret_cast<const BackendPixelRect*>(&rect));
            }
        )
    );
}
extern "C" void lollib_backend_destroyBackendScreenAnalyzer(BackendScreenAnalyzer* screenAnalyzer)
{
    delete reinterpret_cast<ScreenAnalyzer*>(screenAnalyzer);
}
extern "C" BackendScreenResolution lollib_backend_getMode(BackendScreenAnalyzer* screenAnalyzer)
{
    return reinterpret_cast<ScreenAnalyzer*>(screenAnalyzer)->getMode();
}
extern "C" int32_t lollib_backend_hasModeChanged(BackendScreenAnalyzer* screenAnalyzer)
{
    return reinterpret_cast<ScreenAnalyzer*>(screenAnalyzer)->hasModeChanged();
}
extern "C" void lollib_backend_setCaptureRect(BackendScreenAnalyzer* screenAnalyzer, const BackendCaptureRect *captureRect)
{
    reinterpret_cast<ScreenAnalyzer*>(screenAnalyzer)->setCaptureRect(*captureRect);
}
extern "C" LolStats lollib_backend_analyzeScreenshot(BackendScreenAnalyzer* screenAnalyzer)
{
    return reinterpret_cast<ScreenAnalyzer*>(screenAnalyzer)->analyzeScreenshot();
}


extern "C" int32_t lollib_backend_pixelRect_getHight(const BackendPixelRect * rect){
    return reinterpret_cast<const PixelRect*>(rect)->getHight();
}
extern "C" int32_t lollib_backend_pixelRect_getWidth(const BackendPixelRect * rect){
    return reinterpret_cast<const PixelRect*>(rect)->getWidth();
}
extern "C" BackendColor lollib_backend_pixelRect_getColor(const BackendPixelRect * rect, int32_t row, int32_t column){
    return reinterpret_cast<const PixelRect*>(rect)->getColor(row, column);
}