#pragma once

#include "backend_interface.h"

#include "DxObj.hpp"
#include <functional>
#include <d3d9.h>

class PixelRect;

class ScreenAnalyzer
{
public:
  typedef std::function<void(const PixelRect &)> AnalysisFunction;

  explicit ScreenAnalyzer();
  explicit ScreenAnalyzer(ScreenAnalyzer &) = delete;
  explicit ScreenAnalyzer(ScreenAnalyzer &&) = delete;
  ScreenAnalyzer &operator=(ScreenAnalyzer) = delete;

  BackendScreenResolution getMode();

  void analyzeScreenshot(AnalysisFunction analyzeFunction);

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
};
