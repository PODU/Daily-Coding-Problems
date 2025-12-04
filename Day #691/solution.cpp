// Day 691: Bitwise AND of all integers in [M, N].
// Approach: the result is the common binary prefix of M and N. Shift both right
// until equal, then shift back. Time O(log N), Space O(1).
#include <bits/stdc++.h>
using namespace std;

long long rangeBitwiseAnd(long long m, long long n) {
    int shift = 0;
    while (m < n) { m >>= 1; n >>= 1; ++shift; }
    return m << shift;
}

int main() {
    cout << rangeBitwiseAnd(5, 7) << "\n";    // 4
    cout << rangeBitwiseAnd(12, 15) << "\n";  // 12
    return 0;
}
