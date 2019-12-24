#include <fstream>
#include "../lib/IntcodeComputer.hpp"

int main() {
  std::vector<std::pair<size_t, int>> replacements{ std::make_pair<size_t, int>(1, 12), std::make_pair<size_t, int>(2, 2) };
  IntcodeComputer computer{ "./day02/data.txt", replacements };

  if (!computer.isReady()) {
    exit(1);
  }

  computer.run();
  std::cout << "Result " << computer.getIntcode(0) << std::endl;
}
