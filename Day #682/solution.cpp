// add_subtract: chainable builder. Each operator()(x) alternates +/-.
// value() returns the running alternating sum. O(n) time, O(1) space.
#include <iostream>

struct AddSub {
    long long total;
    int sign; // applied to next argument
    AddSub(long long first) : total(first), sign(1) {}
    AddSub(long long t, int s) : total(t), sign(s) {}
    AddSub operator()(long long x) const { return AddSub(total + sign * x, -sign); }
    long long value() const { return total; }
};

AddSub add_subtract(long long first) { return AddSub(first); }

int main() {
    std::cout << add_subtract(7).value() << "\n";            // 7
    std::cout << add_subtract(1)(2)(3).value() << "\n";      // 0
    std::cout << add_subtract(-5)(10)(3)(9).value() << "\n"; // 11
    return 0;
}
