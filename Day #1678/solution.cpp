// Day 1678: Integer division without / * %. Subtract largest shifted multiple of
// divisor each round (doubling). Time O(log(quotient)), Space O(1).
#include <bits/stdc++.h>
using namespace std;

long divide(long a, long b) {
    long q = 0;
    while (a >= b) {
        long temp = b, mult = 1;
        while (a >= (temp << 1)) { temp <<= 1; mult <<= 1; }
        a -= temp; q += mult;
    }
    return q;
}

int main() {
    cout << divide(43, 8) << "\n"; // 5
    return 0;
}
