// Bitwise AND of range [M,N] = common binary prefix; shift both right until equal, then back. O(log N) time, O(1) space.
#include <bits/stdc++.h>
using namespace std;

unsigned rangeBitwiseAnd(unsigned m, unsigned n) {
    int shift = 0;
    while (m < n) { m >>= 1; n >>= 1; ++shift; }
    return m << shift;
}

int main() {
    cout << rangeBitwiseAnd(5, 7) << "\n";
    return 0;
}
