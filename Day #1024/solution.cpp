// Day 1024: Reverse all 32 bits of a 32-bit integer.
// Approach: swap-mask reversal in O(log 32) = O(1) time, O(1) space.
#include <bits/stdc++.h>
using namespace std;

uint32_t reverseBits(uint32_t n) {
    n = ((n & 0xFFFF0000u) >> 16) | ((n & 0x0000FFFFu) << 16);
    n = ((n & 0xFF00FF00u) >> 8)  | ((n & 0x00FF00FFu) << 8);
    n = ((n & 0xF0F0F0F0u) >> 4)  | ((n & 0x0F0F0F0Fu) << 4);
    n = ((n & 0xCCCCCCCCu) >> 2)  | ((n & 0x33333333u) << 2);
    n = ((n & 0xAAAAAAAAu) >> 1)  | ((n & 0x55555555u) << 1);
    return n;
}

string grouped(uint32_t n) {
    string s;
    for (int i = 31; i >= 0; --i) {
        s += ((n >> i) & 1u) ? '1' : '0';
        if (i % 4 == 0 && i != 0) s += ' ';
    }
    return s;
}

int main() {
    uint32_t x = 0xF0F0F0F0u;
    cout << grouped(reverseBits(x)) << "\n";
    return 0;
}
