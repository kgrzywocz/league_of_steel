#pragma once

#include "ScreenAnalyzer.hpp"
#include "PixelRow.hpp"
#include "Color.hpp"
#include "LolStats.h"
#include <stdint.h>
#include <d3d9.h>

class LolLib
{
public:
    explicit LolLib()
    {
        adjustBarsOnScreen();
    }

    D3DDISPLAYMODE getMode()
    {
        return m_screenAnalyzer.getMode();
    }

    bool hasModeChanged()
    {
        return m_screenAnalyzer.hasModeChanged();
    }

    LolStats getCurrentStats()
    {
        return m_screenAnalyzer.analyzeScreenshot();
    }

    void setHudScaling(float hudGlobalScale)
    {
        m_hudScale = 0.333 * hudGlobalScale + 0.666;
        adjustBarsOnScreen();
    }

private:
    void adjustBarsOnScreen()
    {
        m_captureRect = getBarsPosition(m_screenAnalyzer.getMode());
        m_screenAnalyzer.setCaptureRect(m_captureRect);
    }

    RECT getBarsPosition(const D3DDISPLAYMODE &dispMode)
    {
        RECT pos;
        //682x1031 1094x1044 on 1920x1080
        //455x688 729x709 on 1280x720
        //415x985 748x1011 on 1280x1024
        //729x1038 1071x1067 on 1920x1080 - hud 50% -> 0.83
        //776x1047 1049x1069 on 1920x1080 - hud 0%(0.01) -> 0.66

        pos.left = LONG(0.355 * dispMode.Width);
        pos.right = LONG(0.57 * dispMode.Width);
        pos.top = LONG(0.962 * dispMode.Height);
        pos.bottom = LONG(0.98 * dispMode.Height);

        reScaleForHudScaling(pos, dispMode);

        return pos;
    }

    void reScaleForHudScaling(RECT &pos, const D3DDISPLAYMODE &dispMode)
    {
        auto midWidth = dispMode.Width / 2;
        pos.left = LONG(midWidth - m_hudScale * (midWidth - pos.left));
        pos.right = LONG(midWidth + m_hudScale * (pos.right - midWidth));
        pos.top = LONG(dispMode.Height - m_hudScale * (dispMode.Height - pos.top));
        pos.bottom = LONG(dispMode.Height - m_hudScale * (dispMode.Height - pos.bottom));
    }

    LolStats analyzeScreenshot(const D3DLOCKED_RECT &rc)
    {
        LolStats stats;

        auto len = m_captureRect.right - m_captureRect.left;
        auto h = m_captureRect.bottom - m_captureRect.top;

        PixelRow hp{rc.pBits, len};
        stats.health = analyzeHealth(hp);

        PixelRow mana{rc.pBits, len, rc.Pitch * h};
        stats.mana = analyzeMana(mana);

        stats.hit = analyzeHit(hp, stats.health);

        return stats;
    }

    uint8_t analyzeHealth(const PixelRow &p)
    {
        return analyzeBar(p, [](const Color &c) { return c.isGreen(); });
    }
    uint8_t analyzeHit(const PixelRow &p, uint8_t startFromPercent)
    {
        return analyzeBar(p, [](const Color &c) { return c.isRed(); }, startFromPercent);
    }
    uint8_t analyzeMana(const PixelRow &p)
    {
        return analyzeBar(p, [](const Color &c) { return c.isBlue() || c.isYellow(); });
    }

    uint8_t analyzeBar(const PixelRow &p, std::function<bool(const Color &)> predicate, uint8_t startFromPercent = 0)
    {
        const int max_gap = 20;

        auto len = p.getLen();
        int offset = startFromPercent * len / 100;

        int last = offset;
        for (int j = offset; j < len; j++)
        {
            if (predicate(p.get(j)))
                last = j;
            else if (j > last + max_gap)
                break;
        }

        return uint8_t(100 * (last + 1) / len) - startFromPercent;
    }

private:
    ScreenAnalyzer<LolStats> m_screenAnalyzer{std::bind(&LolLib::analyzeScreenshot, this, std::placeholders::_1)};
    RECT m_captureRect;
    double m_hudScale = 1.0;
};