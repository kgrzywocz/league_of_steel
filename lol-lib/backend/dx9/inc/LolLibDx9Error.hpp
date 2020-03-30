#pragma once

#include <exception>
#include <sstream>

class LolLibDx9Error : public std::exception
{
public:
  LolLibDx9Error(int result, const std::string &file, size_t line, std::string expr = "")
  {
    std::stringstream ss;
    ss << "ERROR 0x" << std::hex << result << "(" << std::dec << result << ")";
    ss << " in file:" << file << ":" << line << std::endl;
    ss << expr;
    description = ss.str();
  }

  const char *what() const throw()
  {
    return description.c_str();
  }

private:
  std::string description;
};
