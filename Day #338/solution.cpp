// Next higher integer with same number of set bits (snoob bit-twiddle).
// O(1) time, O(1) space.
#include <bits/stdc++.h>
using namespace std;

unsigned nextSameBits(unsigned n) {
    unsigned smallest = n & (~n + 1u); // n & -n
    unsigned ripple = n + smallest;
    unsigned ones = n ^ ripple;
    ones = (ones >> 2) / smallest;
    return ripple | ones;
}

int main() {
    cout << nextSameBits(6u) << '\n'; // 9
    return 0;
}
