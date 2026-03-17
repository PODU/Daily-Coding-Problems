// Count total set bits in 1..N using per-bit cycle formula. O(log N) time, O(1) space.
#include <bits/stdc++.h>
using namespace std;

long long countSetBits(long long n) {
    long long total = 0;
    for (long long p = 2; p <= 2 * n; p <<= 1) {
        long long full = ((n + 1) / p) * (p / 2);
        long long rem = max(0LL, (n + 1) % p - p / 2);
        total += full + rem;
    }
    return total;
}

int main() {
    cout << countSetBits(5) << "\n";
    return 0;
}
