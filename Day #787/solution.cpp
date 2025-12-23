// Next bigger integer with same popcount via bit-hack (Gosper's hack). O(1) time, O(1) space.
#include <bits/stdc++.h>
using namespace std;

unsigned nextSamePopcount(unsigned n) {
    if (n == 0) return 0;
    unsigned c = n & -n;          // lowest set bit
    unsigned r = n + c;           // ripple carry
    unsigned ones = ((n ^ r) >> 2) / c; // shifted-in ones
    return r | ones;
}

int main() {
    cout << nextSamePopcount(6) << "\n"; // 0110 -> 1001 = 9
    return 0;
}
