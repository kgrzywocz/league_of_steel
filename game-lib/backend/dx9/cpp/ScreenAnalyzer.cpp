#include "ScreenAnalyzer.hpp"

#include "PixelRect.hpp"
#include "DxObj.hpp"
#include "LolLibDx9Error.hpp"
#include <stdint.h>
#include <functional>
#include <d3d9.h>
#pragma comment(lib, "d3d9.lib")

#define RES_CHECK(__expr)                                          \
    {                                                              \
        auto hr = (__expr);                                        \
        if (FAILED(hr))                                            \
        {                                                          \
            throw LolLibDx9Error(hr, __FILE__, __LINE__, #__expr); \
        }                                                          \
    }

ScreenAnalyzer::ScreenAnalyzer()
{
    IDirect3D9Ex *d3d;
    RES_CHECK(Direct3DCreate9Ex(D3D_SDK_VERSION, &d3d));
    m_d3d.set(d3d);

    initDxDevice();
}

BackendScreenResolution ScreenAnalyzer::getMode()
{
    D3DDISPLAYMODE dx_mode;
    RES_CHECK(m_d3d->GetAdapterDisplayMode(adapter, &dx_mode));

    BackendScreenResolution mode;
    mode.width = dx_mode.Width;
    mode.height = dx_mode.Height;
    return mode;
}

void ScreenAnalyzer::initDxDevice()
{
    RES_CHECK(m_d3d->GetAdapterDisplayMode(adapter, &m_mode));
    if (m_mode.Format != D3DFMT_X8R8G8B8)
        throw LolLibDx9Error(-1, __FILE__, __LINE__, "display mode.Format != D3DFMT_X8R8G8B8");

    IDirect3DDevice9Ex *device = nullptr;
    D3DPRESENT_PARAMETERS parameters = {0};
    parameters.Windowed = TRUE;
    parameters.BackBufferCount = 1;
    parameters.BackBufferHeight = m_mode.Height;
    parameters.BackBufferWidth = m_mode.Width;
    parameters.SwapEffect = D3DSWAPEFFECT_DISCARD;
    parameters.hDeviceWindow = NULL;
    RES_CHECK(m_d3d->CreateDeviceEx(adapter, D3DDEVTYPE_HAL, NULL, D3DCREATE_SOFTWARE_VERTEXPROCESSING, &parameters, NULL, &device));
    m_device.set(device);

    IDirect3DSurface9 *surface = nullptr;
    RES_CHECK(m_device->CreateOffscreenPlainSurface(m_mode.Width, m_mode.Height, D3DFMT_A8R8G8B8, D3DPOOL_SYSTEMMEM, &surface, nullptr));
    m_surface.set(surface);
}

void ScreenAnalyzer::releaseDxDevice()
{
    m_surface.set(NULL);
    m_device.set(NULL);
}

void ScreenAnalyzer::reinitDxDevice()
{
    releaseDxDevice();
    initDxDevice();
}
bool ScreenAnalyzer::hasModeChanged()
{
    D3DDISPLAYMODE mode;
    RES_CHECK(m_d3d->GetAdapterDisplayMode(adapter, &mode));
    return mode.Height != m_mode.Height || mode.Width != m_mode.Width;
}

void ScreenAnalyzer::analyzeScreenshot(AnalysisFunction analyzeFunction)
{
    if (hasModeChanged())
        reinitDxDevice();

    D3DLOCKED_RECT rc;
    RECT captureRect{0};
    captureRect.right = m_mode.Width;
    captureRect.bottom = m_mode.Height;

    RES_CHECK(m_device->GetFrontBufferData(0, m_surface));
    RES_CHECK(m_surface->LockRect(&rc, &captureRect, 0));

    analyzeFunction(PixelRect{rc, captureRect});

    RES_CHECK(m_surface->UnlockRect());
}
