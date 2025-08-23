// Day 161: Reverse the 32 bits of an integer by shifting bits out of the input
// into the result. Time O(32), Space O(1).
#include <bits/stdc++.h>
using namespace std;

uint32_t reverseBits(uint32_t n) {
    uint32_t res = 0;
    for (int i = 0; i < 32; i++) {
        res = (res << 1) | (n & 1);
        n >>= 1;
    }
    return res;
}

string toGroups(uint32_t n) {
    string s;
    for (int i = 31; i >= 0; i--) {
        s += ((n >> i) & 1) ? '1' : '0';
        if (i % 4 == 0 && i != 0) s += ' ';
    }
    return s;
}

int main() {
    uint32_t n = 0xF0F0F0F0; // 1111 0000 ... repeated
    cout << toGroups(reverseBits(n)) << "\n";
    return 0;
}
