// Fast (binary) exponentiation: square-and-multiply. Time O(log y), Space O(1).
#include <iostream>
using namespace std;

long long fastPow(long long x, long long y) {
    long long result = 1;
    bool neg = y < 0;
    if (neg) y = -y; // for integers, x^-y is fractional; we handle |y| here.
    while (y > 0) {
        if (y & 1) result *= x;
        x *= x;
        y >>= 1;
    }
    return result; // (caller may invert for negative exponents)
}

int main() {
    cout << fastPow(2, 10) << endl; // 1024
    return 0;
}
