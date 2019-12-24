#pragma once
#include <vector>
#include <iostream>
#include <fstream>
#include <string>
#include <functional>

class IntcodeComputer {
public:
  IntcodeComputer(const char path[]);
  IntcodeComputer(const char path[], std::vector<std::pair<size_t, int>> replacements);

  void printCodes() const;

  bool isReady() {
    return ready;
  }

  void run();

  int getIntcode(size_t index);

private:
  std::vector<int> intcodes{};
  size_t pointer{ 0 };
  bool ready{ false };
  bool debugMode{ false };

  void printDebug(int opcode, int operand1, int operand2, int operand3);
  bool updateIntcode(size_t index, int value);
};
