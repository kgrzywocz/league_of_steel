#pragma once

#include <typeinfo>
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

template <typename T, typename Klass>
T callSafely_member(Klass obj, std::function<T()> cpp_fun) throw()
{
    if (!obj)
    {
        printf("%s not initialized in call of %s\n", typeid(Klass).name(), typeid(T).name());
        return T{};
    }
    return callSafely(cpp_fun);
}
