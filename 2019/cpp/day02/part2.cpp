#include <fstream>
#include "../lib/IntcodeComputer.hpp"

int main() {
  IntcodeComputer computer{ "./day02/data.txt" };

  if (!computer.isReady()) {
    exit(1);
  }

  int sought{ 19690720 };
  int result{};

  for (size_t i = 0; i <= 99; i++) {
    if (result != 0) break;
    for (size_t j = 0; j <= 99; j++) {
      computer.reset();
      computer.loadNounAndVerb(i, j);
      computer.run();
      if (sought == computer.getIntcode(0)) {
        result = (100 * i) + j;
      }
    }
  }

  std::cout << "Result: " << result << std::endl;
}
