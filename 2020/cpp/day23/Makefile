CXX=clang++
CXXFLAGS=-g -std=c++2a -Wall -pedantic

SRC=$(wildcard *.cpp)
OBJ=$(SRC:%.cpp=%.o)

part1: part1.o
		$(CXX) -o part1.out $^

part2: part2.o
		$(CXX) -o part2.out $^

all:
		$(part1)
		$(part2)

%.o: %.c
		$(CXX) $@ -c $<

clean:
		rm -f *.o
		rm part1.out
		rm part2.out
