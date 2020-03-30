#pragma once

#include "Color.hpp"
#include "PixelRow.hpp"
#include "PixelRect.hpp"
#include "LolStats.h"
#include <functional>

class PixelRectAnalyzer
{
public:
    explicit PixelRectAnalyzer(){}

    LolStats getStats(const BackendPixelRect &p_rect)
    {
        const auto & rect = reinterpret_cast<const PixelRect &>(p_rect);

        LolStats stats;

        auto hp = rect.getPixelRow(0);
        stats.health = analyzeHealth(hp);

        auto mana = rect.getPixelRow(rect.getNumberOfRows());
        stats.mana = analyzeMana(mana);

        stats.hit = analyzeHit(hp, stats.health);

        return stats;
    }

private:
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
            if (predicate(Color(p.get(j))))
                last = j;
            else if (j > last + max_gap)
                break;
        }

        return uint8_t(100 * (last + 1) / len) - startFromPercent;
    }
};
