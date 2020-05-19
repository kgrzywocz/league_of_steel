#include "ScreenAnalyzer.hpp"

#include "PixelRect.hpp"
#include <string>
#include <windows.h>
#pragma comment(lib, "User32.lib")
#pragma comment(lib, "Gdi32.lib")
#include <stdio.h>

ScreenAnalyzer::ScreenAnalyzer(AnalysisFunction analyzeFunction)
    : m_analyzeFunction{analyzeFunction}
{
}

LolStats ScreenAnalyzer::analyzeScreenshot()
{
    auto hwnd = FindWindow(NULL, "League of Legends (TM) Client");
    //printf("LOL window = %d\n", hwnd);

    auto hdc = GetDC(hwnd);
    const auto &res = m_analyzeFunction(PixelRect{hdc, m_captureRect});
    ReleaseDC(hwnd, hdc);

    return res;
}
