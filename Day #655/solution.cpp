// Reverse 32 bits: iterate 32 times, shift result left, OR in LSB of input, shift input right.
// Time O(32)=O(1), space O(1). Uses unsigned 32-bit arithmetic.
#include <cstdint>
#include <iostream>
#include <string>

uint32_t reverseBits(uint32_t x) {
    uint32_t result = 0;
    for (int i = 0; i < 32; ++i) {
        result = (result << 1) | (x & 1u);
        x >>= 1;
    }
    return result;
}

int main() {
    uint32_t input = 0xF0F0F0F0u;
    uint32_t out = reverseBits(input);
    std::string s;
    for (int i = 31; i >= 0; --i) {
        s += ((out >> i) & 1u) ? '1' : '0';
        if (i % 4 == 0 && i != 0) s += ' ';
    }
    std::cout << s << "\n";
    return 0;
}
