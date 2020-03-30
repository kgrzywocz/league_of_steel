#pragma once

#include "PixelRow.hpp"
#include <d3d9.h>

class PixelRect
{
public:
  explicit PixelRect(const D3DLOCKED_RECT &rc, const RECT &captureRect)
      : m_rc(rc),
        m_captureRect(captureRect)
  {
  }

  PixelRow getPixelRow(int row) const
  {
    auto len = m_captureRect.right - m_captureRect.left;
    return PixelRow{m_rc.pBits, len, m_rc.Pitch * row};
  }

  int getNumberOfRows() const
  {
    return m_captureRect.bottom - m_captureRect.top;
  }

private:
  const D3DLOCKED_RECT &m_rc;
  const RECT &m_captureRect;
};