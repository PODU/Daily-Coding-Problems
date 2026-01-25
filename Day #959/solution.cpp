// Day 959: total number of set bits over all integers in [1, N].
// Per-bit counting of full cycles plus remainder. Time O(log N), Space O(1).
#include <bits/stdc++.h>
using namespace std;

long long countSetBits(long long n) {
    long long total = 0;
    for (int i = 0; (1LL << i) <= n; ++i) {
        long long cycle = 1LL << (i + 1);
        long long half = cycle >> 1;
        total += (n + 1) / cycle * half;
        long long rem = (n + 1) % cycle;
        total += max(0LL, rem - half);
    }
    return total;
}

int main() {
    cout << countSetBits(5) << "\n"; // 1+1+2+1+2 = 7
    return 0;
}
