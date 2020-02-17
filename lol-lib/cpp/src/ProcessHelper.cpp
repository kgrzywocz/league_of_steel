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
    int pid;
    TCHAR filename[MAX_PATH];
    
    pid = getProcessId(exeName);
    printf("%d\n", pid);
    //std::string res;

    HANDLE processHandle = OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, FALSE, pid);
    
    if (processHandle != NULL)
    {
        if (GetModuleFileNameEx(processHandle, NULL, filename, MAX_PATH) != 0)
        {
            //res =filename;
            return filename;
        }
        CloseHandle(processHandle);
    }
    return "";
}