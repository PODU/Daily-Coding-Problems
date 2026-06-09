// Branchless select: mask = -b (all-ones if b=1 else 0); result = (x & mask) | (y & ~mask). O(1) time/space.
#include <cstdint>
#include <iostream>
using namespace std;

int32_t select_(int b, int32_t x, int32_t y) {
    uint32_t mask = (uint32_t)(-(int32_t)b); // 0xFFFFFFFF if b=1, 0 if b=0
    return (int32_t)(((uint32_t)x & mask) | ((uint32_t)y & ~mask));
}

int main() {
    cout << select_(1, 42, 99) << "\n";
    cout << select_(0, 42, 99) << "\n";
    return 0;
}
