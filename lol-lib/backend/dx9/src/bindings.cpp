#include "backend_interface.h"

#include "ScreenAnalyzer.hpp"

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