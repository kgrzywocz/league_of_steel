#pragma once

#include <d3d9.h>

class BarsPosition
{
public:
    RECT get(const D3DDISPLAYMODE &dispMode)
    {
        RECT pos;
        //682x1031 1094x1044 on 1920x1080
        //455x688 729x709 on 1280x720
        //415x985 748x1011 on 1280x1024
        //729x1038 1071x1067 on 1920x1080 - hud 50% -> 0.83
        //776x1047 1049x1069 on 1920x1080 - hud 0%(0.01) -> 0.66
        //381x744 575x761 on 1024x768 - hud 0%(0.01) -> 0.66

        pos.left = LONG(0.355 * dispMode.Width);
        pos.right = LONG(0.57 * dispMode.Width);
        pos.top = LONG(0.962 * dispMode.Height);
        pos.bottom = LONG(0.98 * dispMode.Height);

        reScaleForHudScaling(pos, dispMode);

        return pos;
    }

    void setHudScaling(float hudGlobalScale)
    {
        m_hudScale = 0.333 * hudGlobalScale + 0.666;
    }

private:
    void reScaleForHudScaling(RECT &pos, const D3DDISPLAYMODE &dispMode)
    {
        auto midWidth = dispMode.Width / 2;
        pos.left = LONG(midWidth - m_hudScale * (midWidth - pos.left));
        pos.right = LONG(midWidth + m_hudScale * (pos.right - midWidth));
        pos.top = LONG(dispMode.Height - m_hudScale * (dispMode.Height - pos.top));
        pos.bottom = LONG(dispMode.Height - m_hudScale * (dispMode.Height - pos.bottom));
    }

    double m_hudScale = 1.0;
};