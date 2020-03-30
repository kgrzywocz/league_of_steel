#pragma once

#include "backend_interface.h"


class ScreenAnalyzerWrapper
{
public:

  explicit ScreenAnalyzerWrapper(FrontendAnalysisFunction analyzeFunction)
  {
    m_backendScreenAnalyzer= lollib_backend_createBackendScreenAnalyzer(analyzeFunction);
  }
  ~ScreenAnalyzerWrapper(){
    lollib_backend_destroyBackendScreenAnalyzer(m_backendScreenAnalyzer);
  }
  ScreenAnalyzerWrapper(ScreenAnalyzerWrapper&)=delete;
  ScreenAnalyzerWrapper(ScreenAnalyzerWrapper&&)=delete;
  ScreenAnalyzerWrapper& operator=(ScreenAnalyzerWrapper)=delete;

  BackendScreenResolution getMode()
  {
      return lollib_backend_getMode(m_backendScreenAnalyzer);
  }

  bool hasModeChanged()
  {
      return lollib_backend_hasModeChanged(m_backendScreenAnalyzer);
  }

  void setCaptureRect(const BackendCaptureRect &captureRect)
  {
       lollib_backend_setCaptureRect(m_backendScreenAnalyzer, &captureRect);
  }

  LolStats analyzeScreenshot()
  {
      return lollib_backend_analyzeScreenshot(m_backendScreenAnalyzer);
  }

private:
    BackendScreenAnalyzer* m_backendScreenAnalyzer;
};
