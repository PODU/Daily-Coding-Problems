// Swap even/odd bits of 8-bit int: ((n&0xAA)>>1)|((n&0x55)<<1), masked to 8 bits.
// Time: O(1), Space: O(1).
#include <bits/stdc++.h>
using namespace std;

string swapBits(const string& bin) {
    int n = stoi(bin, nullptr, 2);
    int r = (((n & 0xAA) >> 1) | ((n & 0x55) << 1)) & 0xFF;
    string out;
    for (int i = 7; i >= 0; --i) out += ((r >> i) & 1) ? '1' : '0';
    return out;
}

int main() {
    cout << swapBits("10101010") << "\n";
    cout << swapBits("11100010") << "\n";
    return 0;
}
