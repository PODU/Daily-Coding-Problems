// Fast (binary) exponentiation by squaring on a 64-bit result.
// Time: O(log y), Space: O(1).
#include <bits/stdc++.h>
using namespace std;

long long fastPow(long long x, long long y) {
    long long result = 1;
    while (y > 0) {
        if (y & 1) result *= x;
        x *= x;
        y >>= 1;
    }
    return result;
}

int main() {
    cout << fastPow(2, 10) << "\n";
    return 0;
}
