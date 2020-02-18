#pragma once

#include <string>

bool isProcessRunning(const std::string &processName);
std::string getProcessRunningPath(const std::string &exeName);