// Day 1773: Count ordered positive pairs (a,b) with a+b=M, a^b=N.
// Use a+b=(a^b)+2*(a&b): s=(M-N)/2 must satisfy (s&N)==0; answer=2^popcount(N)
// minus the all-or-nothing assignments when s==0. O(1) time, O(1) space.
#include <bits/stdc++.h>
using namespace std;

long long countPairs(long long M, long long N) {
    if (M - N < 0 || ((M - N) & 1)) return 0;
    long long s = (M - N) / 2;          // s == a & b
    if (s & N) return 0;                // carry bits must be disjoint from xor bits
    if (N == 0) return (M > 0 && (M % 2 == 0)) ? 1 : 0; // only (M/2, M/2)
    long long ways = 1LL << __builtin_popcountll(N);
    if (s == 0) ways -= 2;              // drop a=0 and b=0 to keep both positive
    return ways;
}

int main() {
    cout << countPairs(6, 4) << "\n"; // (1,5),(5,1) -> 2
    return 0;
}
