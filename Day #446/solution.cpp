// Day 446: Power of four test in O(1) (better than O(log N)).
// Power of two (single set bit) AND that bit sits at an even position.
#include <bits/stdc++.h>
using namespace std;

bool isPowerOfFour(unsigned int n) {
    // n>0, exactly one bit set, and bit on an even index (mask 0x55555555)
    return n != 0 && (n & (n - 1)) == 0 && (n & 0x55555555u) != 0;
}

int main() {
    for (unsigned int n : {1u, 4u, 8u, 16u, 64u, 0u, 5u, 256u})
        cout << n << " -> " << (isPowerOfFour(n) ? "true" : "false") << "\n";
    // 1->true 4->true 8->false 16->true 64->true 0->false 5->false 256->true
    return 0;
}
