// Day 451: nth Fibonacci number in O(1) space.
// Iterative rolling pair. Time O(n), Space O(1).
#include <iostream>
using namespace std;

long long fib(int n) {
    if (n < 2) return n;
    long long a = 0, b = 1;
    for (int i = 2; i <= n; ++i) {
        long long c = a + b;
        a = b;
        b = c;
    }
    return b;
}

int main() {
    cout << fib(10) << endl; // 55
    return 0;
}
