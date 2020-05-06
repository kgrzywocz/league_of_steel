#pragma once

#include "backend_interface.h"
#include <windows.h>
#include <functional>

class PixelRect;

class ScreenAnalyzer
{
public:
  typedef std::function<LolStats(const PixelRect &)> AnalysisFunction;

  explicit ScreenAnalyzer(AnalysisFunction analyzeFunction);
  explicit ScreenAnalyzer(ScreenAnalyzer &) = delete;
  explicit ScreenAnalyzer(ScreenAnalyzer &&) = delete;
  ScreenAnalyzer &operator=(ScreenAnalyzer) = delete;

  BackendScreenResolution getDisplayMode()
  {
    RECT rect;
    GetWindowRect(GetDesktopWindow(),&rect);

    BackendScreenResolution mode;
    mode.width = rect.right;
    mode.height = rect.bottom;
    return mode;
  }

  BackendScreenResolution getMode()
  {
    return m_mode;
  }

  bool hasModeChanged()
  {
    return getDisplayMode().height != m_mode.height || getDisplayMode().height != m_mode.height;
  }

  void setCaptureRect(const BackendCaptureRect &captureRect)
  {
    m_captureRect.top = captureRect.top;
    m_captureRect.bottom = captureRect.bottom;
    m_captureRect.left = captureRect.left;
    m_captureRect.right = captureRect.right;
  }

  LolStats analyzeScreenshot();

private:
  BackendScreenResolution m_mode;

  AnalysisFunction m_analyzeFunction;
  RECT m_captureRect{0};
};
