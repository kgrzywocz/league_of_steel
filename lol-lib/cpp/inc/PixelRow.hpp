#pragma once

#include "Color.hpp"
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
  Color get(int pos) const
  {
    return Color(&m_bytes[pos * BITSPERPIXEL / 8]);
  }
  int getLen() const
  {
    return m_len;
  }

private:
  uint8_t *m_bytes;
  int m_len;
};
