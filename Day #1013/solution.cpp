// Bitwise AND of all ints in [M,N] = common binary prefix of M and N.
// Shift both right until equal, then shift back. Time: O(log N), Space: O(1).
#include <bits/stdc++.h>
using namespace std;

long long rangeAnd(long long m, long long n) {
    int shift = 0;
    while (m < n) { m >>= 1; n >>= 1; shift++; }
    return m << shift;
}

int main() {
    cout << "AND(5, 7) = " << rangeAnd(5, 7) << "\n";      // 4
    cout << "AND(12, 15) = " << rangeAnd(12, 15) << "\n";  // 12
    return 0;
}
