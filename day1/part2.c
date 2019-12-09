#include <stdio.h>
#include <stdlib.h>

int getFuelRequirement(int module) {
  int requiredFuel = (module / 3) - 2;
  return requiredFuel < 0
    ? 0
    : requiredFuel + getFuelRequirement(requiredFuel);
}

int main() {
  FILE *file = fopen("./day1/data.txt", "r");
  if (file == NULL) {
    exit(1);
  }

  char chunk[10];
  size_t len = sizeof(chunk);

  int requiredFuel = 0;

  while (fgets(chunk, len, file) != NULL) {
    requiredFuel += getFuelRequirement(atoi(chunk));
    chunk[0] = '\0';
  }

  printf("%d\n", requiredFuel);

  fclose(file);
}
