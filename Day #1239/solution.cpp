// Integer division without * / %. Subtract doubled divisor. Time O(log^2 q).
#include <bits/stdc++.h>
using namespace std;

long long divide(long long dividend, long long divisor) {
    long long quotient = 0;
    while (dividend >= divisor) {
        long long temp = divisor, multiple = 1;
        while ((temp << 1) <= dividend) { temp <<= 1; multiple <<= 1; }
        dividend -= temp;
        quotient += multiple;
    }
    return quotient;
}

int main() {
    cout << divide(43, 5) << "\n";
    cout << divide(100, 10) << "\n";
    return 0;
}
