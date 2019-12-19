#include "ProcessHelper.hpp"

#include <windows.h>
#include <tlhelp32.h>

bool isProcessRunning(const std::string &processName)
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

    CloseHandle(snapshot);
    return exists;
}
