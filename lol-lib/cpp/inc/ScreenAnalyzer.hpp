#pragma once

#include "PixelRow.hpp"
#include "DxObj.hpp"
#include "LolLibError.hpp"
#include <stdint.h>
#include <functional>
#include <cwchar>
#include <d3d9.h>
#pragma comment(lib, "d3d9.lib")

#define RES_CHECK(__expr)                                 \
  {                                                       \
    auto hr = (__expr);                                   \
    if (FAILED(hr))                                       \
    {                                                     \
      throw LolLibError(hr, __FILE__, __LINE__, #__expr); \
    }                                                     \
  }

template <typename AnalysisResult>
class ScreenAnalyzer
{
public:
  typedef std::function<AnalysisResult(const D3DLOCKED_RECT &)> AnalysisFunction;
  explicit ScreenAnalyzer(AnalysisFunction analyzeFunction)
      : m_analyzeFunction{analyzeFunction}
  {
    IDirect3D9Ex *d3d;
    RES_CHECK(Direct3DCreate9Ex(D3D_SDK_VERSION,&d3d));
    m_d3d.set(d3d);
    RES_CHECK(m_d3d->GetAdapterDisplayMode(adapter, &m_mode));
    if (m_mode.Format != D3DFMT_X8R8G8B8)
      throw LolLibError(-1, __FILE__, __LINE__, "display mode.Format != D3DFMT_X8R8G8B8");

    IDirect3DDevice9Ex *device = nullptr;
    D3DPRESENT_PARAMETERS parameters = {0};
    parameters.Windowed = TRUE;
    parameters.BackBufferCount = 1;
    parameters.BackBufferHeight = m_mode.Height;
    parameters.BackBufferWidth = m_mode.Width;
    parameters.SwapEffect = D3DSWAPEFFECT_DISCARD;
    parameters.hDeviceWindow = NULL;
    RES_CHECK(m_d3d->CreateDeviceEx(adapter, D3DDEVTYPE_HAL, NULL, D3DCREATE_SOFTWARE_VERTEXPROCESSING, &parameters, NULL,&device));
    m_device.set(device);

    IDirect3DSurface9 *surface = nullptr;
    RES_CHECK(m_device->CreateOffscreenPlainSurface(m_mode.Width, m_mode.Height, D3DFMT_A8R8G8B8, D3DPOOL_SYSTEMMEM, &surface, nullptr));
    m_surface.set(surface);
  }

  D3DDISPLAYMODE getMode()
  {
    return m_mode;
  }
  
  bool hasModeChanged()
  {
    D3DDISPLAYMODE mode;
    RES_CHECK(m_d3d->GetAdapterDisplayMode(adapter, &mode));
    return mode.Height != m_mode.Height && mode.Width != m_mode.Width;
  }

  void setCaptureRect(const RECT &captureRect)
  {
    m_captureRect = captureRect;
  }

  AnalysisResult analyzeScreenshot()
  {
    D3DLOCKED_RECT rc;

    RES_CHECK(m_device->GetFrontBufferData(0, m_surface));
    RES_CHECK(m_surface->LockRect(&rc, &m_captureRect, 0));

    const auto &res = m_analyzeFunction(rc);

    RES_CHECK(m_surface->UnlockRect());

    return res;
  }

private:
  const UINT adapter = D3DADAPTER_DEFAULT;

  DxObj<IDirect3D9Ex *> m_d3d;
  DxObj<IDirect3DDevice9Ex *> m_device;
  DxObj<IDirect3DSurface9 *> m_surface;

  D3DDISPLAYMODE m_mode;

  AnalysisFunction m_analyzeFunction;
  RECT m_captureRect{0};
};
