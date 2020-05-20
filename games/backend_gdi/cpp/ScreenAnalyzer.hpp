#pragma once

#include "backend_interface.h"
#include <windows.h>
#include <functional>

class PixelRect;

class ScreenAnalyzer
{
public:
  typedef std::function<void(const PixelRect &)> AnalysisFunction;

  explicit ScreenAnalyzer();
  explicit ScreenAnalyzer(ScreenAnalyzer &) = delete;
  explicit ScreenAnalyzer(ScreenAnalyzer &&) = delete;
  ScreenAnalyzer &operator=(ScreenAnalyzer) = delete;

  BackendScreenResolution getDisplayMode()
  {
    RECT rect;
    GetWindowRect(GetDesktopWindow(), &rect);

    BackendScreenResolution mode;
    mode.width = rect.right;
    mode.height = rect.bottom;
    return mode;
  }

  BackendScreenResolution getMode()
  {
    return getDisplayMode();
  }

  void setCaptureRect(const BackendCaptureRect &captureRect)
  {
    m_captureRect.top = captureRect.top;
    m_captureRect.bottom = captureRect.bottom;
    m_captureRect.left = captureRect.left;
    m_captureRect.right = captureRect.right;
  }

  void analyzeScreenshot(AnalysisFunction analyzeFunction);

private:
  RECT m_captureRect{0};
};
