#pragma once

#include "backend_interface.h"
#include <stdint.h>
#include <windows.h>

class PixelRect
{
public:
  explicit PixelRect(HDC hdc, uint32_t width, uint32_t hight)
      : m_hdc(hdc),
        m_width(width),
        m_hight(hight)
  {
  }

  BackendColor getColor(uint32_t x, uint32_t y) const
  {
    auto pixel = GetPixel(m_hdc, x, y);

    return makeColor(pixel);
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

  HDC m_hdc;

  const uint32_t m_width;
  const uint32_t m_hight;

  static BackendColor makeColor(const uint32_t pixel)
  {
    BackendColor res;
    res.b = (int)(pixel & 0x00FF0000) >> 16;
    res.g = (int)(pixel & 0x0000FF00) >> 8;
    res.r = (int)(pixel & 0x000000FF);
    res.a = 0;

    //printf("%2x%2x%2x ", res.r, res.g, res.b);

    return res;
  }
};