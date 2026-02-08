// Demonstrates three kinds of polymorphism: ad-hoc (overloading), parametric
// (templates), subtype (virtual dispatch via base pointer). O(1) demo.
#include <iostream>
#include <string>
#include <vector>
#include <memory>

// Ad-hoc: function overloading
int add(int a, int b) { return a + b; }
std::string add(const std::string& a, const std::string& b) { return a + b; }

// Parametric: generic template returning the first element of any container
template <typename T>
T first(const std::vector<T>& v) { return v[0]; }

// Subtype: base class with virtual method overridden by subclasses
struct Animal {
    virtual std::string speak() const = 0;
    virtual ~Animal() = default;
};
struct Dog : Animal { std::string speak() const override { return "Woof"; } };
struct Cat : Animal { std::string speak() const override { return "Meow"; } };

int main() {
    std::cout << "Ad-hoc polymorphism: same name, different argument types (overloading).\n";
    std::cout << "Ad-hoc: add(2,3)=" << add(2, 3) << ", add(\"a\",\"b\")=" << add(std::string("a"), std::string("b")) << "\n";

    std::cout << "Parametric polymorphism: one generic definition works for any type.\n";
    std::cout << "Parametric: first([1,2,3])=" << first(std::vector<int>{1, 2, 3})
              << ", first([\"x\",\"y\"])=" << first(std::vector<std::string>{"x", "y"}) << "\n";

    std::cout << "Subtype polymorphism: a base reference dispatches to the subclass override.\n";
    std::unique_ptr<Animal> dog = std::make_unique<Dog>();
    std::unique_ptr<Animal> cat = std::make_unique<Cat>();
    std::cout << "Subtype: Dog says " << dog->speak() << ", Cat says " << cat->speak() << "\n";
    return 0;
}
