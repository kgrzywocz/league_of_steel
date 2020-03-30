#pragma once

#include <stdint.h>

extern "C" {

typedef struct _BackendCaptureRectTag
{
    int32_t    left;
    int32_t    top;
    int32_t    right;
    int32_t    bottom;
} BackendCaptureRect;

typedef struct _BackendScreenResolutionTag{
    uint32_t Width;
    uint32_t Height;
}BackendScreenResolution;

typedef struct _BackendColor
{
  uint8_t B = 0;
  uint8_t G = 0;
  uint8_t R = 0;
  uint8_t A = 0;
} BackendColor;

} //extern "C" 