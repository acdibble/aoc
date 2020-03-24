#include <stdio.h>
#include <stdlib.h>

int main() {
  FILE *file = fopen("./day1/data.txt", "r");
  if (file == NULL) {
    exit(1);
  }

  char chunk[10];
  size_t len = sizeof(chunk);

  int requiredFuel = 0;

  while (fgets(chunk, len, file) != NULL) {
    requiredFuel += (atoi(chunk) / 3) - 2;
    chunk[0] = '\0';
  }

  printf("%d\n", requiredFuel);

  fclose(file);
}
