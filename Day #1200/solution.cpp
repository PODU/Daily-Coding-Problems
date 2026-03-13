// Reverse all 32 bits of an unsigned integer by shifting LSB of input into LSB-first of result.
// Time: O(1) (32 steps); Space: O(1).
#include <iostream>
#include <cstdint>
#include <string>

uint32_t reverseBits(uint32_t x) {
    uint32_t r = 0;
    for (int i = 0; i < 32; i++) {
        r = (r << 1) | (x & 1u);
        x >>= 1;
    }
    return r;
}

std::string toGrouped(uint32_t x) {
    std::string s;
    for (int i = 31; i >= 0; i--) {
        s += ((x >> i) & 1u) ? '1' : '0';
        if (i % 4 == 0 && i != 0) s += ' ';
    }
    return s;
}

int main() {
    uint32_t input = 0xF0F0F0F0u;
    std::cout << "Input:  " << toGrouped(input) << std::endl;
    std::cout << toGrouped(reverseBits(input)) << std::endl;
    return 0;
}
