#pragma once

#include "backend_interface.h"

class BarsPosition
{
public:
    BackendCaptureRect get(const BackendScreenResolution &dispMode)
    {
        BackendCaptureRect pos;
        //682x1031 1094x1065 on 1920x1080
        //455x688 729x709 on 1280x720
        //415x985 748x1011 on 1280x1024
        //729x1038 1071x1067 on 1920x1080 - hud 50% -> 0.83
        //776x1047 1049x1069 on 1920x1080 - hud 0%(0.01) -> 0.66
        //381x744 575x761 on 1024x768 - hud 0%(0.01) -> 0.66
        //245x572 474x591 on 800x600
        //297x581 449x594 on 800x600 hud 0
        //314x732 607x757 on 1024x768
        //354x824 684x851 on 1152x864

        auto ratio = ((double)dispMode.width / dispMode.height) / 1.77777777777;
        ratio = (ratio + 2) / 3;

        pos.left = int32_t(ratio * 0.356 * dispMode.width);
        pos.right = int32_t(ratio * 0.57 * dispMode.width);                           //bar len
        pos.right = int32_t(dispMode.width - (ratio * (dispMode.width - pos.right))); //space on rigth
        pos.bottom = int32_t((1 - 0.015 * ratio) * dispMode.height)-1;
        pos.top = pos.bottom - int32_t(ratio * ratio * 0.025 * dispMode.height)+1;

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
        int midWidth = dispMode.width / 2;
        pos.left = int32_t(midWidth - m_hudScale * (midWidth - pos.left));
        pos.right = int32_t(midWidth + m_hudScale * (pos.right - midWidth));
        pos.top = int32_t(dispMode.height - m_hudScale * (dispMode.height - pos.top));
        pos.bottom = int32_t(dispMode.height - m_hudScale * (dispMode.height - pos.bottom));
    }

    double m_hudScale = 1.0;
};