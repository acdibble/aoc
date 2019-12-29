#include <fstream>
#include <iostream>

int getFuelRequirement(int module) {
  int requiredFuel{ (module / 3) - 2 };
  return requiredFuel < 0
    ? 0
    : requiredFuel + getFuelRequirement(requiredFuel);
}

int main() {
  std::ifstream file{ "./day1/data.txt" };

  int total{};
  std::string line{};
  while (file >> line) {
    total += getFuelRequirement(std::stoi(line));
  }

  file.close();

  std::cout << total << std::endl;
}
