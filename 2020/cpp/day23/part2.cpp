#include <iostream>
#include <memory>
#include <vector>

// constexpr int cups[] = {3, 8, 9, 1, 2, 5, 4, 6, 7};
constexpr int cups[] = {1, 6, 7, 2, 4, 8, 3, 5, 9};

constexpr size_t CUP_COUNT{1'000'000};
constexpr size_t MOVE_COUNT{10'000'000};

class Node
{
public:
  int value;
  Node *next;

  Node(int value, Node *next) : value{value}, next{next} {};
};

class LinkedList
{
public:
  Node *pointer{nullptr};
  std::vector<Node *> map{};
  int highestValue{};

  LinkedList(const std::vector<int> inputs)
  {
    map.reserve(inputs.size() + 1);
    Node *tail{nullptr};
    for (auto it{inputs.crbegin()}; it != inputs.crend(); it++)
    {
      if (*it > highestValue)
        highestValue = *it;

      pointer = new Node(*it, pointer);

      map[*it] = pointer;
      if (tail == nullptr)
        tail = pointer;
    }

    tail->next = pointer;
  }

  ~LinkedList()
  {
    pointer = nullptr;
    for (size_t i{}; i < map.size(); i++)
    {
      delete map[i];
    }
  }

  void insertAfter(Node *value, Node *destination)
  {
    value->next = destination->next;
    destination->next = value;
  }

  Node *removeAfterPointer()
  {
    auto afterPointer{pointer->next};
    pointer->next = afterPointer->next;
    afterPointer->next = nullptr;
    return afterPointer;
  }

  void advancePointer()
  {
    pointer = pointer->next;
  }

  void doMove()
  {
    Node *removedValues[3]{removeAfterPointer(), removeAfterPointer(), removeAfterPointer()};

    int value{pointer->value};
    Node *destination{nullptr};

    do
    {
      value -= 1;
      if (value <= 0)
        value = highestValue;
      destination = map[value];
    } while (destination == removedValues[0] || destination == removedValues[1] || destination == removedValues[2]);

    for (const auto &value : removedValues)
    {
      insertAfter(value, destination);
      destination = value;
    }

    advancePointer();
  }
};

int main(void)
{
  std::vector<int> inputs{};

  for (size_t i{}; i < CUP_COUNT; i++)
  {
    if (i < 9)
    {
      inputs.push_back(cups[i]);
    }
    else
    {
      inputs.push_back(i + 1);
    }
  }

  LinkedList list{inputs};

  for (size_t i{}; i < MOVE_COUNT; i++)
  {
    list.doMove();
  }

  auto node1{list.map[1]};

  std::cout << static_cast<long long>(node1->next->value) * static_cast<long long>(node1->next->next->value) << std::endl;
}
