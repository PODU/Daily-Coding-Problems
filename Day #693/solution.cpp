// Day 693: Swap adjacent (even/odd) bits of an unsigned 8-bit integer.
// Approach: one-liner masks. Odd bits 0x55 shift left, even bits 0xAA shift right.
// Time O(1), Space O(1).
#include <bits/stdc++.h>
using namespace std;

unsigned char swapBits(unsigned char n) {
    return ((n & 0xAA) >> 1) | ((n & 0x55) << 1);
}

string bin8(unsigned char n) { return bitset<8>(n).to_string(); }

int main() {
    cout << bin8(swapBits((unsigned char)0b10101010)) << "\n"; // 01010101
    cout << bin8(swapBits((unsigned char)0b11100010)) << "\n"; // 11010001
    return 0;
}
