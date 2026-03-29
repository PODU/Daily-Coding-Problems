// Day 1280: Swap adjacent bit pairs of an 8-bit unsigned integer.
// One-liner: shift odd bits up, even bits down. Time O(1), Space O(1).
#include <bits/stdc++.h>
using namespace std;

unsigned char swapBits(unsigned char n) {
    return ((n & 0xAA) >> 1) | ((n & 0x55) << 1);
}

string toBin(unsigned char n) {
    return bitset<8>(n).to_string();
}

int main() {
    for (string in : {"10101010", "11100010"}) {
        unsigned char n = (unsigned char)stoi(in, nullptr, 2);
        cout << toBin(swapBits(n)) << "\n";
    }
    return 0;
}
