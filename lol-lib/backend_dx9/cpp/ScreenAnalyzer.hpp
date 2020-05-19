#pragma once

#include "backend_interface.h"

#include "DxObj.hpp"
#include <functional>
#include <d3d9.h>

class PixelRect;

class ScreenAnalyzer
{
public:
  typedef std::function<LolStats(const PixelRect &)> AnalysisFunction;

  explicit ScreenAnalyzer(AnalysisFunction analyzeFunction);
  explicit ScreenAnalyzer(ScreenAnalyzer &) = delete;
  explicit ScreenAnalyzer(ScreenAnalyzer &&) = delete;
  ScreenAnalyzer &operator=(ScreenAnalyzer) = delete;

  BackendScreenResolution getMode();

  void setCaptureRect(const BackendCaptureRect &captureRect)
  {
    m_captureRect.top = captureRect.top;
    m_captureRect.bottom = captureRect.bottom;
    m_captureRect.left = captureRect.left;
    m_captureRect.right = captureRect.right;
  }

  LolStats analyzeScreenshot();

private:
  void initDxDevice();
  void releaseDxDevice();
  void reinitDxDevice();
  bool hasModeChanged();

  const UINT adapter = D3DADAPTER_DEFAULT;

  DxObj<IDirect3D9Ex *> m_d3d;
  DxObj<IDirect3DDevice9Ex *> m_device;
  DxObj<IDirect3DSurface9 *> m_surface;
  D3DDISPLAYMODE m_mode;

  AnalysisFunction m_analyzeFunction;
  RECT m_captureRect{0};
};
