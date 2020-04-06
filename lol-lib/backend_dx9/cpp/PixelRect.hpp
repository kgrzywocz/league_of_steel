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

  BackendColor getColor(int row, int column) const
  {
    auto pRow = m_pBits + m_pitch * row;

    return makeColor(&pRow[column * BYTESPERPIXEL]);
  }

  int getHight() const
  {
    return m_hight;
  }
  int getWidth() const
  {
    return m_width;
  }

private:
  static const uint8_t BYTESPERPIXEL = 32/8;

  const uint8_t * const m_pBits;
  const int32_t m_pitch;

  const int32_t m_width;
  const int32_t m_hight;

  static BackendColor makeColor(const uint8_t bytes[4])
  {
    BackendColor res;
    res.B = bytes[0];
    res.G = bytes[1];
    res.R = bytes[2];
    res.A = bytes[3];
    return res;
  }
};