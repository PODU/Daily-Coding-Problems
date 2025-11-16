// Day 610: Integer division of positive ints without / , * , or %.
// Approach: repeated doubling subtraction (binary long division). Time O(log^2), Space O(1).
#include <bits/stdc++.h>
using namespace std;

long long divide(long long dividend, long long divisor) {
    long long q = 0;
    while (dividend >= divisor) {
        long long temp = divisor, mult = 1;
        while (dividend >= (temp << 1)) { temp <<= 1; mult <<= 1; }
        dividend -= temp;
        q += mult;
    }
    return q;
}

int main() {
    cout << divide(10, 3) << "\n"; // 3
    cout << divide(43, 8) << "\n"; // 5
    return 0;
}
