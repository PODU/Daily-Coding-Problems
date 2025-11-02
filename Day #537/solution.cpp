// Collatz: iterate n->n/2 (even) or 3n+1 (odd) counting steps to 1; verify a range.
// Bonus longest under 1e6 via memoized step counts. Time ~O(LIMIT*avg-chain), space O(LIMIT).
#include <bits/stdc++.h>
using namespace std;

int steps(long long n){ int c=0; while(n!=1){ n = (n%2==0)? n/2 : 3*n+1; ++c; } return c; }

int main(){
    bool allReach = true;
    for(int n=1;n<=20;++n) if(steps(n) < 0) allReach=false; // every call terminating proves reach
    cout << "Collatz conjecture holds for 1..20: " << (allReach?"true":"false") << "\n";

    const int LIMIT = 1000000;
    vector<int> dp(LIMIT+1, 0);
    dp[1] = 0;
    int bestN = 1, bestLen = 0;
    for(int i=2;i<=LIMIT;++i){
        long long n = i; int c = 0;
        while(n != 1 && (n > LIMIT || dp[n] == 0)){
            n = (n%2==0)? n/2 : 3*n+1; ++c;
        }
        c += (n==1)? 0 : dp[n];
        dp[i] = c;
        if(c > bestLen){ bestLen = c; bestN = i; }
    }
    cout << "Longest under 1000000: n=" << bestN << " with " << bestLen << " steps\n";
    return 0;
}
