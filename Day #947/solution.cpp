// Day 947: smallest sparse number (no two adjacent set bits) >= N.
// Repeatedly fix the lowest adjacent-1 pair by rounding up. Time O(log N), Space O(1).
#include <bits/stdc++.h>
using namespace std;

unsigned long long smallestSparse(unsigned long long n) {
    while ((n & (n >> 1)) != 0ULL) {
        unsigned long long m = n & (n >> 1);      // bits where i and i+1 both set
        int p = __builtin_ctzll(m);               // lowest such position
        unsigned long long step = 1ULL << (p + 1);
        n = (n + step) & ~(step - 1);             // round up, clearing low bits
    }
    return n;
}

int main() {
    cout << smallestSparse(21) << "\n"; // 21 is already sparse -> 21
    cout << smallestSparse(22) << "\n"; // 22 (10110) -> 32 (100000)
    return 0;
}
