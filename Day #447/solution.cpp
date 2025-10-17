// Day 447: Integer pow via exponentiation by squaring. O(log y) time, O(1) space.
#include <bits/stdc++.h>
using namespace std;

// returns x^y for integer y (handles negative y as a double)
double ipow(long long x, long long y) {
    if (y < 0) return 1.0 / ipow(x, -y);
    long long result = 1, base = x;
    while (y > 0) {
        if (y & 1) result *= base;
        base *= base;
        y >>= 1;
    }
    return (double)result;
}

int main() {
    cout << (long long)ipow(2, 10) << "\n"; // 1024
    cout << (long long)ipow(3, 5) << "\n";  // 243
    cout << ipow(2, -2) << "\n";            // 0.25
    return 0;
}
