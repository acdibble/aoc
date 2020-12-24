#include <iostream>
#include <memory>
#include <unordered_map>

// constexpr int cups[] = {3, 8, 9, 1, 2, 5, 4, 6, 7};
constexpr int cups[] = {1, 6, 7, 2, 4, 8, 3, 5, 9};

class Node
{
public:
  int value;
  std::shared_ptr<Node> next;

  Node(int value, std::shared_ptr<Node> next) : value{value}, next{next} {};
};

class LinkedList
{
public:
  std::shared_ptr<Node> pointer{nullptr};
  std::unordered_map<int, std::shared_ptr<Node>> map{};
  int highestValue{};

  LinkedList(const int *inputs, const size_t length)
  {
    std::shared_ptr<Node> tail{nullptr};
    for (int i{(int)length - 1}; i >= 0; i--)
    {
      if (inputs[i] > highestValue)
        highestValue = inputs[i];

      pointer = std::make_shared<Node>(inputs[i], pointer);
      map[inputs[i]] = pointer;
      if (tail == nullptr)
        tail = pointer;
    }

    tail->next = pointer;
  }

  void insertAfter(int value, int destination)
  {
    auto node{map[destination]};
    auto newNode{std::make_shared<Node>(value, node->next)};
    map[value] = newNode;
    node->next = newNode;
  }

  int removeAfterPointer()
  {
    auto afterPointer{pointer->next};
    pointer->next = afterPointer->next;
    map.erase(afterPointer->value);
    return afterPointer->value;
  }

  void advancePointer()
  {
    pointer = pointer->next;
  }

  void doMove()
  {
    print();
    int removedValues[3]{removeAfterPointer(), removeAfterPointer(), removeAfterPointer()};

    int destination{pointer->value - 1};
    if (destination <= 0)
      destination = highestValue;
    while (map.find(destination) == map.end())
    {
      destination -= 1;
      if (destination <= 0)
        destination = highestValue;
    }

    for (const auto &value : removedValues)
    {
      insertAfter(value, destination);
      destination = value;
    }

    advancePointer();
  }

  void print() const
  {
    auto current{pointer};
    do
    {
      std::cout << current->value << " ";
      current = current->next;
    } while (current != pointer);
    std::cout << std::endl;
  }
};

int main(void)
{
  LinkedList list{cups, sizeof(cups) / sizeof(cups[0])};

  for (size_t i{}; i < 100; i++)
  {
    list.doMove();
  }

  auto start{list.map.at(1)->next};

  while (start->value != 1)
  {
    std::cout << start->value;
    start = start->next;
  }

  std::cout << std::endl;
}
