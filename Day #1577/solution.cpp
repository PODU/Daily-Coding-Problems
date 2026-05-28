// Day 1577: Smallest sparse number (no adjacent 1 bits) >= N.
// Repeatedly fix the lowest pair of adjacent ones by clearing low bits and carrying.
// Time: O((log N)^2) worst-case (well under O(N log N)); Space: O(1).
#include <bits/stdc++.h>
using namespace std;

unsigned long long smallestSparse(unsigned long long n) {
    while (n & (n >> 1)) {                 // while two adjacent ones exist
        int i = 0;
        while (!(((n >> i) & 1ULL) && ((n >> (i + 1)) & 1ULL))) i++;
        unsigned long long mask = (1ULL << (i + 2)) - 1;   // bits 0..i+1
        n = (n & ~mask) + (1ULL << (i + 2));               // clear them, carry
    }
    return n;
}

int main() {
    cout << smallestSparse(21) << "\n"; // 21
    return 0;
}
