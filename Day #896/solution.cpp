// Integer division without / * %: subtract largest shifted multiple of divisor.
// Bit-shifting. Time O((log n)^2), Space O(1).
#include <bits/stdc++.h>
using namespace std;

long long divide(long long dividend, long long divisor) {
    long long quotient = 0;
    while (dividend >= divisor) {
        long long temp = divisor, multiple = 1;
        while (dividend >= (temp << 1)) {
            temp <<= 1;
            multiple <<= 1;
        }
        dividend -= temp;
        quotient += multiple;
    }
    return quotient;
}

int main() {
    cout << divide(43, 8) << "\n";
    cout << divide(100, 9) << "\n";
    return 0;
}
