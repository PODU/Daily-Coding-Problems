// Day 1144: count positive pairs (a,b) with a+b=M, a^b=N.
// a+b = (a^b) + 2*(a&b) => and=(M-N)/2; valid iff M>=N, even, and&N==0.
// Ordered pairs = 2^popcount(N), minus 2 if and==0 (excludes a=0/b=0). O(1).
#include <bits/stdc++.h>
using namespace std;

long long countPairs(long long M, long long N) {
    if (M < N || ((M - N) & 1)) return 0;
    long long a_and = (M - N) / 2;
    if (a_and & N) return 0;
    long long cnt = 1LL << __builtin_popcountll(N);
    if (a_and == 0) cnt -= 2;       // remove a=0 and b=0 assignments
    return cnt < 0 ? 0 : cnt;
}

int main() {
    cout << countPairs(10, 4) << "\n"; // 2  -> (7,3) and (3,7)
    return 0;
}
