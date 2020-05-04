#pragma once

#include "Color.hpp"
#include "PixelRectWrapper.hpp"
#include "backend_interface.h"
#include <functional>

class PixelRectAnalyzer
{
public:
    explicit PixelRectAnalyzer(const BackendPixelRect &rect)
    : m_rect(rect),
    m_rowLen(m_rect.getWidth())
    {
    }

    LolStats getStats()
    {
        LolStats stats;

        auto hp = 0;
        auto mana = m_rect.getHight();

        stats.health = analyzeHealth(hp);        
        stats.mana = analyzeMana(mana);
        stats.hit = analyzeHit(hp, stats.health);

        return stats;
    }

private:
    const PixelRectWrapper m_rect;
    const int m_rowLen;

    uint8_t analyzeHealth(const int row)
    {
        return analyzeBar(row, [](const Color &c) { return c.isGreen(); });
    }
    uint8_t analyzeHit(const int row, uint8_t startFromPercent)
    {
        return analyzeBar(row, [](const Color &c) { return c.isRed(); }, startFromPercent);
    }
    uint8_t analyzeMana(const int row)
    {
        return analyzeBar(row, [](const Color &c) { return c.isBlue() || c.isYellow(); });
    }

    uint8_t analyzeBar(const int row, std::function<bool(const Color &)> predicate, uint8_t startFromPercent = 0)
    {
        const int max_gap = 20;

        int offset = startFromPercent * m_rowLen / 100;

        int last = offset;
        for (int j = offset; j < m_rowLen; j++)
        {
            if (predicate(Color(m_rect.getColor(row, j))))
                last = j;
            else if (j > last + max_gap)
                break;
        }

        return uint8_t(100 * (last + 1) / m_rowLen) - startFromPercent;
    }
};
