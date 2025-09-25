// Count ordered (a,b), a+b=M, a^b=N. a+b=(a^b)+2*(a&b) => c=(M-N)/2; valid if c&N==0.
// Count=2^popcount(N), minus 2 if M==N (excludes a=0 or b=0). Time O(1), Space O(1).
#include <bits/stdc++.h>
using namespace std;

long long countPairs(long long M, long long N) {
    long long diff = M - N;
    if (diff < 0 || (diff & 1)) return 0;
    long long c = diff / 2;
    if (c & N) return 0;
    long long count = 1LL << __builtin_popcountll(N);
    if (M == N) count -= 2;
    return count < 0 ? 0 : count;
}

int main() {
    cout << countPairs(10, 4) << "\n";
    return 0;
}
