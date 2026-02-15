// Variadic alternating add/subtract: first arg seeds the total, then the
// rest alternate +, -, +, ... O(n) time, O(1) space.
#include <iostream>
#include <initializer_list>

int add_subtract(std::initializer_list<int> args) {
    int result = 0, sign = 1;
    bool first = true;
    for (int v : args) {
        if (first) { result = v; first = false; }
        else { result += sign * v; sign = -sign; }
    }
    return result;
}

int main() {
    std::cout << "add_subtract(7) = "             << add_subtract({7})          << "\n";
    std::cout << "add_subtract(1)(2)(3) = "       << add_subtract({1, 2, 3})    << "\n";
    std::cout << "add_subtract(-5)(10)(3)(9) = "  << add_subtract({-5,10,3,9})  << "\n";
}
