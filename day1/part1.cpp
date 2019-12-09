#include <fstream>
#include <iostream>

int getFuelRequirement(int module) {
  return (module / 3) - 2;
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
