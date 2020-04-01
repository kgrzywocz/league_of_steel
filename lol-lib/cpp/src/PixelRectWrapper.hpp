#pragma once

#include "backend_interface.h"

class PixelRectWrapper
{
public:
  explicit PixelRectWrapper(const BackendPixelRect &rect)     
    : m_rect(rect)
  {
  }

  BackendColor getColor(int row, int column) const
  {
    return lollib_backend_pixelRect_getColor(&m_rect,row,column);
  }

  int getHight() const
  {
    return lollib_backend_pixelRect_getHight(&m_rect);
  }
  int getWidth() const
  {
    return lollib_backend_pixelRect_getWidth(&m_rect);
  }
private:
    const BackendPixelRect &m_rect;
};