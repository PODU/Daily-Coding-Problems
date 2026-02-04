// Day 1021: Swap even and odd bits of an 8-bit integer.
// Approach: ((n & 0xAA) >> 1) | ((n & 0x55) << 1).  Time O(1), Space O(1).
#include <bits/stdc++.h>
using namespace std;

unsigned swapBits(unsigned n) { return (((n & 0xAA) >> 1) | ((n & 0x55) << 1)) & 0xFF; }

string toBin(unsigned n) {
    string s;
    for (int i = 7; i >= 0; --i) s += ((n >> i) & 1) ? '1' : '0';
    return s;
}

int main() {
    vector<string> in = {"10101010", "11100010"};
    for (auto& b : in) {
        unsigned n = (unsigned)stoul(b, nullptr, 2);
        cout << toBin(swapBits(n)) << "\n";
    }
}
