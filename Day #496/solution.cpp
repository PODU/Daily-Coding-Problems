// Day 496: Total set bits across 1..N.
// For each bit position, count how many numbers in [0,N] have it set using the
// periodic pattern. O(log N) time, O(1) space.
#include <bits/stdc++.h>
using namespace std;

long long countSetBits(long long n) {
    long long total = 0;
    for (long long bit = 1; bit <= n; bit <<= 1) {
        long long full = n + 1;          // count of integers in [0, n]
        long long cycle = bit << 1;      // period for this bit
        total += (full / cycle) * bit;   // full cycles each contribute `bit` ones
        long long rem = full % cycle;    // partial cycle
        total += max(0LL, rem - bit);    // ones in the leftover
    }
    return total;
}

int main() {
    cout << countSetBits(5) << "\n";  // 7  (1+1+2+1+2)
    cout << countSetBits(16) << "\n"; // 33
    return 0;
}
