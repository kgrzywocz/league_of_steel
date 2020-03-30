#pragma once

#include "backend_interface.h"

class BarsPosition
{
public:
    BackendCaptureRect get(const BackendScreenResolution &dispMode)
    {
        BackendCaptureRect pos;
        //682x1031 1094x1044 on 1920x1080
        //455x688 729x709 on 1280x720
        //415x985 748x1011 on 1280x1024
        //729x1038 1071x1067 on 1920x1080 - hud 50% -> 0.83
        //776x1047 1049x1069 on 1920x1080 - hud 0%(0.01) -> 0.66
        //381x744 575x761 on 1024x768 - hud 0%(0.01) -> 0.66

        auto ratio = ((double)dispMode.Width / dispMode.Height) / 1.77777777777;
        ratio = (ratio +2)/3;

        pos.left = int32_t(ratio * 0.356 * dispMode.Width);
        pos.right = int32_t(ratio * 0.57 * dispMode.Width);
        pos.right = int32_t(dispMode.Width - (ratio * (dispMode.Width - pos.right)));
        pos.top = int32_t(0.962 * dispMode.Height);
        pos.bottom = int32_t(0.98 * dispMode.Height);

        reScaleForHudScaling(pos, dispMode);

        return pos;
    }

    void setHudScaling(float hudGlobalScale)
    {
        m_hudScale = 0.333 * hudGlobalScale + 0.666;
    }

private:
    void reScaleForHudScaling(BackendCaptureRect &pos, const BackendScreenResolution &dispMode)
    {
        int midWidth = dispMode.Width / 2;
        pos.left = int32_t(midWidth - m_hudScale * (midWidth - pos.left));
        pos.right = int32_t(midWidth + m_hudScale * (pos.right - midWidth));
        pos.top = int32_t(dispMode.Height - m_hudScale * (dispMode.Height - pos.top));
        pos.bottom = int32_t(dispMode.Height - m_hudScale * (dispMode.Height - pos.bottom));
    }

    double m_hudScale = 1.0;
};