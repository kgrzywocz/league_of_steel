#include "ProcessHelper.hpp"

#include <windows.h>
#include <tlhelp32.h>

HANDLE getSnapshot(const std::string &processName)
{
    bool exists = false;
    PROCESSENTRY32 entry;
    entry.dwSize = sizeof(PROCESSENTRY32);

    HANDLE snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, NULL);

    if (Process32First(snapshot, &entry))
        while (Process32Next(snapshot, &entry))
        {
            if (!strcmp(entry.szExeFile, processName.c_str()))
            {
                exists = true;
                break;
            }
        }
    
    if (!exists)
    {
        CloseHandle(snapshot);
        return NULL;
    }
    return snapshot;
}

bool isProcessRunning(const std::string &processName)
{
    auto snapshot = getSnapshot(processName);

    if (snapshot)
    {
        CloseHandle(snapshot);
        return true;
    }
    return false;
}


std::string getProcessRunningPath(const std::string &exeName)
{
    auto snapshot = getSnapshot(exeName);
    MODULEENTRY32 moduleEntry;

    if (snapshot)
    {
        if (Module32First(snapshot, &moduleEntry))
        {
            return moduleEntry.szExePath;
        }
        CloseHandle(snapshot);
    }
    return "";
}