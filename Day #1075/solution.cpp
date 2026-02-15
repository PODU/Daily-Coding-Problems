// Iterative Fibonacci with two rolling variables. O(n) time, O(1) space.
#include <iostream>
int fib(int n) {
    if (n == 0) return 0;
    long long a = 0, b = 1;
    for (int i = 2; i <= n; ++i) { long long c = a + b; a = b; b = c; }
    return (int)b;
}
int main() {
    for (int i = 0; i <= 10; ++i) std::cout << fib(i) << (i < 10 ? " " : "\n");
    std::cout << "fib(10) = " << fib(10) << "\n";
}
