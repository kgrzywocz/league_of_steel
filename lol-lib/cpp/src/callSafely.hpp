#pragma once

#include <exception>
#include <functional>

template <typename T>
T callSafely(std::function<T()> cpp_fun) throw()
{
    try
    {
        return cpp_fun();
    }
    catch (std::exception &e)
    {
        printf("%s\n", e.what());
    }
    catch (...)
    {
        printf("lollib other exception\n");
    }
    return T{};
}

template <typename T>
T callSafely_member(std::function<T()> cpp_fun) throw()
{
    if (!lollib)
    {
        printf("lollib not initialized\n");
        return T{};
    }
    return callSafely(cpp_fun);
}
