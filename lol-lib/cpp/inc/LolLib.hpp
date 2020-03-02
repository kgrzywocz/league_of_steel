#pragma once

#include "ScreenAnalyzer.hpp"
#include "BarsPosition.hpp"
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
        m_barsPosition.setHudScaling(hudGlobalScale);
        adjustBarsOnScreen();
    }

private:
    void adjustBarsOnScreen()
    {
        m_captureRect = m_barsPosition.get(m_screenAnalyzer.getMode());
        m_screenAnalyzer.setCaptureRect(m_captureRect);
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
    BarsPosition m_barsPosition;
    RECT m_captureRect;
};