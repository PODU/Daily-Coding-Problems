// Day 1303: Next larger integer with the same number of set bits (snoob / "same number of one bits").
// Bit-hack: add lowest set bit, then re-insert the moved bits at the bottom. O(1) time.
#include <bits/stdc++.h>
using namespace std;

unsigned long long nextSameBits(unsigned long long n) {
    unsigned long long c = n & (~n + 1);      // lowest set bit
    unsigned long long r = n + c;             // ripple carry
    unsigned long long ones = ((n ^ r) >> 2) / c; // moved bits, shifted down
    return r | ones;
}

int main() {
    cout << nextSameBits(6) << endl; // 9
    return 0;
}
