// Reverse the 32 bits of a 32-bit integer by shifting result left and pulling
// the lowest bit of the input each iteration. Time O(1) (32 iters), Space O(1).
#include <iostream>
#include <cstdint>
#include <string>
using namespace std;

uint32_t reverseBits(uint32_t n) {
    uint32_t result = 0;
    for (int i = 0; i < 32; ++i) {
        result = (result << 1) | (n & 1u);
        n >>= 1;
    }
    return result;
}

string groupNibbles(uint32_t n) {
    string out;
    for (int i = 31; i >= 0; --i) {
        out += ((n >> i) & 1u) ? '1' : '0';
        if (i % 4 == 0 && i != 0) out += ' ';
    }
    return out;
}

int main() {
    uint32_t input = 0xF0F0F0F0u;
    uint32_t rev = reverseBits(input);
    cout << groupNibbles(rev) << "\n";
    return 0;
}
