#pragma once

#include "backend_interface.h"
#include <stdint.h>
#include <windows.h>

class PixelRect
{
public:
  explicit PixelRect(HDC hdc, const RECT &captureRect)
      : m_hdc(hdc),
        m_x(captureRect.left),
        m_y(captureRect.top),
        m_width(captureRect.right - captureRect.left),
        m_hight(captureRect.bottom - captureRect.top)
  {
  }

  BackendColor getColor(int row, int column) const
  {
    printf("%d,%d=", column + m_x, row + m_y);
    auto pixel = GetPixel(m_hdc, column + m_x, row + m_y);

    return makeColor(pixel);
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
  static const uint8_t BYTESPERPIXEL = 32 / 8;

  HDC m_hdc;
  const int32_t m_x;
  const int32_t m_y;

  const int32_t m_width;
  const int32_t m_hight;

  static BackendColor makeColor(const uint32_t pixel)
  {
    BackendColor res;
    res.b = (int)(pixel & 0x00FF0000) >> 16;
    res.g = (int)(pixel & 0x0000FF00) >> 8;
    res.r = (int)(pixel & 0x000000FF);
    res.a = 0;

    printf("%2x%2x%2x ", res.r, res.g, res.b);

    return res;
  }
};