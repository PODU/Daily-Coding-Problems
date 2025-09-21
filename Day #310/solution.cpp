// Day 310: Total set bits over 1..N. Per-bit counting. O(log N).
#include <bits/stdc++.h>
using namespace std;
long long countBits(long long N) {
    long long total = 0;
    for (long long i = 0; (1LL << i) <= N; i++) {
        long long blk = 1LL << (i + 1);
        long long full = (N + 1) / blk * (1LL << i);
        long long rem = max(0LL, (N + 1) % blk - (1LL << i));
        total += full + rem;
    }
    return total;
}
int main() {
    cout << countBits(5) << "\n"; // 7  (1,10,11,100,101 -> 1+1+2+1+2)
}
