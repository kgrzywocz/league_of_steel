#pragma once

#include "backend_interface.h"
#include <stdint.h>
#include <d3d9.h>

class PixelRect
{
public:
  explicit PixelRect(const D3DLOCKED_RECT &rc, const RECT &captureRect)
      : m_pBits{(uint8_t *)rc.pBits},
        m_pitch{rc.Pitch},
        m_width(captureRect.right - captureRect.left),
        m_hight(captureRect.bottom - captureRect.top)
  {
  }

  BackendColor getColor(uint32_t x, uint32_t y) const
  {
    auto pRow = m_pBits + m_pitch * y;

    return makeColor(&pRow[x * BYTESPERPIXEL]);
  }

  uint32_t getHight() const
  {
    return m_hight;
  }
  uint32_t getWidth() const
  {
    return m_width;
  }

private:
  static const uint8_t BYTESPERPIXEL = 32 / 8;

  const uint8_t *const m_pBits;
  const int32_t m_pitch;

  const int32_t m_width;
  const int32_t m_hight;

  static BackendColor makeColor(const uint8_t bytes[4])
  {
    BackendColor res;
    res.b = bytes[0];
    res.g = bytes[1];
    res.r = bytes[2];
    res.a = bytes[3];
    return res;
  }
};