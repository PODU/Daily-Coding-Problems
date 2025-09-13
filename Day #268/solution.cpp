// Day 268: Power of four check in O(1).
// Power of two (N & (N-1))==0 AND single bit at even position (N & 0x55555555). Time O(1), Space O(1).
#include <bits/stdc++.h>
using namespace std;

bool isPowerOfFour(uint32_t n) {
    return n != 0 && (n & (n - 1)) == 0 && (n & 0x55555555u) != 0;
}

int main() {
    uint32_t tests[] = {16, 8, 64, 5};
    for (uint32_t t : tests)
        cout << t << " -> " << (isPowerOfFour(t) ? "True" : "False") << "\n";
    return 0;
}
