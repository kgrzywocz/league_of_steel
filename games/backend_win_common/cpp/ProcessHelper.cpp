#include "ProcessHelper.hpp"

#include <windows.h>
#include <psapi.h>


inline bool ends_with(std::string const & where, std::string const & what)
{
    if (what.size() > where.size())
        return false;
    else
        return std::equal(what.rbegin(), what.rend(), where.rbegin());
}

std::string getProcessFile(DWORD pid)
{
    CHAR filename[MAX_PATH] = "";

    HANDLE processHandle = OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, FALSE, pid);
    if (processHandle != NULL)
    {
        GetModuleFileNameEx(processHandle, NULL, filename, MAX_PATH);
        CloseHandle(processHandle);
    }
    return filename;
}

std::string getProcessRunningPath(const std::string &exeName)
{
    DWORD pids[1024], pidsBytes;
    if ( !EnumProcesses( pids, sizeof(pids), &pidsBytes ) )
        return "";
    auto nPids = pidsBytes / sizeof(pidsBytes);

    while(nPids--)
    {
        auto filePath = getProcessFile(pids[nPids]);
        if (ends_with(filePath, exeName))
            return filePath;
    }
    return "";
}

bool isProcessRunning(const std::string &processName)
{
    return getProcessRunningPath(processName) != "";
}