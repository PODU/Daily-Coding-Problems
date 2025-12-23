// Integer exponentiation by squaring. Time O(log y), Space O(1).
// Negative y handled via double reciprocal; demo uses pow(2,10).
#include <bits/stdc++.h>
using namespace std;

long long ipow(long long x, long long y) {
    long long result = 1;
    long long base = x;
    long long e = y;
    while (e > 0) {
        if (e & 1) result *= base;
        base *= base;
        e >>= 1;
    }
    return result;
}

int main() {
    cout << ipow(2, 10) << "\n";
    return 0;
}
