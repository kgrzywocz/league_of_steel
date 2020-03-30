#pragma once

#include "backend_interface.h"

#include <stdint.h>
#include <cstdlib>

class Color
{
public:
  explicit Color(const BackendColor& color)
  {
    B = color.B;
    G = color.G;
    R = color.R;
    A = color.A;
  }

  bool isRed() const
  {
    return R> B + 50 && R >G + 50;
  }
  bool isGreen() const
  {
    return G> B + 50 && G> R + 50;
  }
  bool isBlue() const
  {
    return B> G + 50 && B>R + 50;
  }
  bool isYellow() const
  {
    return R > B + 50 && G > B + 50 && std::abs(R-G) < 50;
  }

private:
  uint8_t B = 0;
  uint8_t G = 0;
  uint8_t R = 0;
  uint8_t A = 0;
};
