// Max of two ints without if/else/branch/ternary/comparison.
// Use sign bit of (a-b) via 64-bit diff to avoid overflow. O(1) time, O(1) space.
#include <iostream>
#include <cstdint>
using namespace std;

long long maxOf(long long a, long long b) {
    long long d = a - b;             // safe in 64-bit for 32-bit inputs
    long long sign = (d >> 63) & 1;  // 1 if a<b, else 0
    return a - sign * d;             // a - (a-b) = b when a<b, else a
}

int main() {
    cout << "max(3, 7) = " << maxOf(3, 7) << "\n";
    cout << "max(10, 2) = " << maxOf(10, 2) << "\n";
    return 0;
}
