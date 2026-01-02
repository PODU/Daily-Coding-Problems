// Day 841: count positive integer pairs (a,b) with a+b=M and a^b=N.
// Math: a+b = (a^b) + 2*(a&b) => a&b=(M-N)/2; answer = 2^popcount(N) minus zero-cases. O(1) time/space.
#include <bits/stdc++.h>
using namespace std;

long long countPairs(long long M, long long N){
    long long d = M - N;
    if(d < 0 || (d & 1)) return 0;        // need M>=N and (M-N) even
    long long c = d / 2;                  // c = a & b
    if(c & N) return 0;                   // AND and XOR bits cannot overlap
    long long res = 1LL << __builtin_popcountll(N);
    if(c == 0) res -= (N != 0) ? 2 : 1;   // drop pairs containing a 0
    return res < 0 ? 0 : res;
}

int main(){
    long long M = 4, N = 2;
    cout << countPairs(M, N) << "\n";     // 2  -> (1,3) and (3,1)
    return 0;
}
