#pragma once

#include "BarsPosition.hpp"
#include "PixelRectAnalyzer.hpp"
#include "Color.hpp"
#include "ScreenAnalyzerWrapper.hpp"
#include "backend_interface.h"
#include <stdint.h>


class LolLib
{
public:
    explicit LolLib()
    {
        adjustBarsOnScreen();
    }

    int32_t getScreenWidth()
    {
        return m_screenAnalyzer.getMode().width;
    }
    int32_t getScreenHeight()
    {
        return m_screenAnalyzer.getMode().height;
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
        auto captureRect = m_barsPosition.get(m_screenAnalyzer.getMode());
        m_screenAnalyzer.setCaptureRect(captureRect);
    }

    static LolStats analyzeScreenshot(const BackendPixelRect *rect)
    {
        PixelRectAnalyzer pixelRectAnalyzer{*rect};
        return pixelRectAnalyzer.getStats();
    }


private:
    ScreenAnalyzerWrapper m_screenAnalyzer{&LolLib::analyzeScreenshot};
    BarsPosition m_barsPosition;
};