#pragma once

#include "LolLibDx9Error.hpp"

template <typename T>
class DxObj
{
public:
  DxObj() {}
  void set(T obj)
  {
    if (m_obj)
    {
      m_obj->Release();
    }
    m_obj = obj;
  }
  DxObj(DxObj &) = delete;
  DxObj(DxObj &&) = delete;
  DxObj operator=(DxObj) = delete;
  ~DxObj()
  {
    if (m_obj)
    {
      m_obj->Release();
    }
  }
  operator T()
  {
    if (!m_obj)
      throw LolLibDx9Error(-1, __FILE__, __LINE__);
    return m_obj;
  }
  T &operator->()
  {
    if (!m_obj)
      throw LolLibDx9Error(-1, __FILE__, __LINE__);
    return m_obj;
  }

private:
  T m_obj = nullptr;
};