// Bitwise AND of all integers in [M, N] = common binary prefix.
// Shift both right until equal, then shift back. Time O(log N), Space O(1).
#include <bits/stdc++.h>
using namespace std;

long long rangeAnd(long long m, long long n) {
    int shift = 0;
    while (m != n) { m >>= 1; n >>= 1; ++shift; }
    return m << shift;
}

int main() {
    long long M = 5, N = 7;
    cout << "Bitwise AND of [" << M << ", " << N << "] = " << rangeAnd(M, N) << "\n";
    return 0;
}
