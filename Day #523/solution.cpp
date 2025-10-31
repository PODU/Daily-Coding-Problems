// a+b = (a^b) + 2*(a&b). So c=(M-N)/2 must be a valid carry disjoint from N.
// Ordered pairs = 2^popcount(N), minus the two zero-cases when c==0. O(log M).
#include <bits/stdc++.h>
using namespace std;

long long countPairs(long long M, long long N) {
    if (M < N || ((M - N) & 1)) return 0;
    long long c = (M - N) / 2;       // a & b
    if (c & N) return 0;             // AND and XOR bits must be disjoint
    long long ways = 1LL << __builtin_popcountll(N);
    if (c == 0) ways -= 2;           // exclude a=0 and b=0
    return ways < 0 ? 0 : ways;
}

int main() {
    cout << countPairs(10, 4) << "\n"; // 2 -> (3,7) and (7,3)
    return 0;
}
