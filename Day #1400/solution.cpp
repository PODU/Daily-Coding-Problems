// Power of four iff: positive, single set bit (n & (n-1))==0, and that bit sits
// at an even position (n & 0x55555555). O(1) time, O(1) space.
#include <bits/stdc++.h>
using namespace std;

bool isPowerOfFour(unsigned int n) {
    return n != 0 && (n & (n - 1)) == 0 && (n & 0x55555555u) != 0;
}

int main() {
    for (unsigned int n : {1u, 4u, 16u, 5u, 64u, 63u})
        cout << n << " -> " << (isPowerOfFour(n) ? "true" : "false") << "\n";
    return 0;
}
