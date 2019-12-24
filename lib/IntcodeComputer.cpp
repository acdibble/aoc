#include "./IntcodeComputer.hpp"

IntcodeComputer::IntcodeComputer(const char path[]) {
  std::ifstream file{ path };
  if (!file) {
    std::cout << "No file" << std::endl;
  }

  std::string code{};

  while (file.good()) {
    std::getline(file, code, ',');
    intcodes.push_back(std::stoi(code));
  }

  pointer = 0;
  file.close();
  ready = true;
}

IntcodeComputer::IntcodeComputer(const char path[], std::vector<std::pair<size_t, int>> replacements) {
  *this = IntcodeComputer{ path };
  for (const auto &replacement : replacements) {
    if (!updateIntcode(replacement.first, replacement.second)) {
      std::cout << "Couldn't update intcodes with replacements" << std::endl;
      ready = false;
      break;
    }
  }
}

void IntcodeComputer::printCodes() const {
  for (size_t i{ 0 }; i < intcodes.size(); i++) {
    if (i == pointer) std::cout << ">";
    std::cout << intcodes.at(i);
    if (i == pointer) std::cout << "<";
    std::cout << " ";
  }

  std::cout << std::endl;
}

void IntcodeComputer::run() {
  if (!ready) {
    std::cout << "The computer is not ready to run" << std::endl;
    return;
  }

  while(true) {
    if (debugMode) std::cout << "Pointer position: " << pointer << std::endl;
    int verb = intcodes.at(pointer++);
    int operand1 = intcodes.at(pointer++);
    int operand2 = intcodes.at(pointer++);
    int operand3 = intcodes.at(pointer++);
    if (debugMode) std::cout << "opcode: " << verb << std::endl;
    if (debugMode) std::cout << "operand1: " << operand1 << std::endl;
    if (debugMode) std::cout << "operand2: " << operand2 << std::endl;
    if (debugMode) std::cout << "operand3: " << operand3 << std::endl;

    if (verb == 99) {
      break;
    }

    std::function<int(int, int)> op;

    if (verb == 1) {
      op = std::plus<int>();
    } else if (verb == 2) {
      op = std::multiplies<int>();
    } else {
      printDebug(verb, operand1, operand2, operand3);
      ready = false;
      break;
    }

    int result{};
    try {
      result = op(intcodes.at(operand1), intcodes.at(operand2));
    } catch (std::bad_function_call err) {
      printDebug(verb, operand1, operand2, operand3);
    }

    if (!updateIntcode(operand3, result)) {
      printDebug(verb, operand1, operand2, operand3);
      ready = false;
      break;
    }
  }
}

bool IntcodeComputer::updateIntcode(size_t index, int value) {
  if (index >= 0 && index < intcodes.size()) {
    intcodes[index] = value;
    return true;
  }

  return false;
}

void IntcodeComputer::printDebug(int opcode, int operand1, int operand2, int operand3) {
  std::cout << "Encountered an error:" << std::endl;
  std::cout << "Pointer position: " << pointer << std::endl;
  std::cout << "opcode: " << opcode << std::endl;
  std::cout << "operand1: " << operand1 << std::endl;
  std::cout << "operand2: " << operand2 << std::endl;
  std::cout << "operand3: " << operand3 << std::endl;
  printCodes();
}

int IntcodeComputer::getIntcode(size_t index) {
  return intcodes.at(index);
}
