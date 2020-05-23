#include "ScreenAnalyzer.hpp"

#include "PixelRect.hpp"
#include <string>
#include <windows.h>
#pragma comment(lib, "User32.lib")
#pragma comment(lib, "Gdi32.lib")
#include <stdio.h>

ScreenAnalyzer::ScreenAnalyzer()
{
}

void ScreenAnalyzer::analyzeScreenshot(AnalysisFunction analyzeFunction)
{
    auto hwnd = FindWindow(NULL, "League of Legends (TM) Client");
    //printf("LOL window = %d\n", hwnd);

    auto mode = getMode();

    auto hdc = GetDC(hwnd);
    analyzeFunction(PixelRect{hdc, mode.width, mode.height});
    ReleaseDC(hwnd, hdc);
}
