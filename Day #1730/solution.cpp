// Day 1730: Fast integer exponentiation (exponentiation by squaring).
// Square the base and halve the exponent each step. Time: O(log y). Space: O(1).
#include <bits/stdc++.h>
using namespace std;

long long fastPow(long long x, long long y) {
    if (y < 0) { // x^(-y) = 1 / x^y; integer result only for x == 1 or -1.
        long long inv = fastPow(x, -y);
        return inv == 0 ? 0 : 1 / inv;
    }
    long long result = 1, base = x;
    while (y > 0) {
        if (y & 1) result *= base;
        base *= base;
        y >>= 1;
    }
    return result;
}

int main() {
    cout << fastPow(2, 10) << "\n"; // 1024
    return 0;
}
