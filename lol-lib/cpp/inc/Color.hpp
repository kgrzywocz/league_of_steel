#pragma once

#include <stdint.h>
#include <cstdlib>

class Color
{
public:
  explicit Color(uint8_t bytes[4])
  {
    B = bytes[0];
    G = bytes[1];
    R = bytes[2];
    A = bytes[3];
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
