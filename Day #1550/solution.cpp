// Closure capture demo: "buggy" closures share a pointer to one mutable variable (prints 9 x10);
// "fixed" closures capture i by value (prints 0..9). Time O(n), Space O(n).
#include <iostream>
#include <vector>
#include <functional>

int main() {
    // Buggy: every closure captures a reference/pointer to the SAME shared variable.
    std::vector<std::function<int()>> buggy;
    int shared = 0;
    for (int i = 0; i < 10; ++i) {
        shared = i;
        buggy.push_back([&shared]() { return shared; });
    }
    for (auto& f : buggy) std::cout << f() << "\n";

    std::cout << "\n";

    // Fixed: capture i by value at definition time.
    std::vector<std::function<int()>> fixed;
    for (int i = 0; i < 10; ++i) {
        fixed.push_back([i]() { return i; });
    }
    for (auto& f : fixed) std::cout << f() << "\n";

    return 0;
}
