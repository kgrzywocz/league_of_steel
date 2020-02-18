#include "ProcessHelper.hpp"

#include <windows.h>
#include <tlhelp32.h>
#include <psapi.h>

DWORD getProcessId(const std::string &processName)
{
    PROCESSENTRY32 entry;

    HANDLE snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, NULL);

    if (Process32First(snapshot, &entry))
        while (Process32Next(snapshot, &entry))
        {
            if (!strcmp(entry.szExeFile, processName.c_str()))
            {
                CloseHandle(snapshot);
                return entry.th32ProcessID;
            }
        }
    CloseHandle(snapshot);
    return NULL;
}

bool isProcessRunning(const std::string &processName)
{
    return getProcessId(processName) != NULL;
}

std::string getProcessRunningPath(const std::string &exeName)
{
    TCHAR filename[MAX_PATH] = "";
    auto pid = getProcessId(exeName);

    HANDLE processHandle = OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, FALSE, pid);
    if (processHandle != NULL)
    {
        GetModuleFileNameEx(processHandle, NULL, filename, MAX_PATH);
        CloseHandle(processHandle);
    }
    return filename;
}