#pragma once

#include "backend_interface.h"
#include <stdint.h>

class PixelRow
{
public:
  const uint8_t BITSPERPIXEL = 32;
  explicit PixelRow(void *LockedRectBytes, int len, int offset = 0)
      : m_bytes{((uint8_t *)LockedRectBytes) + offset},
        m_len{len}
  {
  }
  BackendColor get(int pos) const
  {
    return makeColor(&m_bytes[pos * BITSPERPIXEL / 8]);
  }
  int getLen() const
  {
    return m_len;
  }

private:
  uint8_t *m_bytes;
  int m_len;

  static BackendColor makeColor(uint8_t bytes[4])
  {
    BackendColor res;
    res.B = bytes[0];
    res.G = bytes[1];
    res.R = bytes[2];
    res.A = bytes[3];
    return res;
  }
};
