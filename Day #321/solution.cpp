// Min steps to reduce N to 1 (N-1 or replace with max factor): DP.
// Time O(N*sqrt(N)), Space O(N).
#include <bits/stdc++.h>
using namespace std;

int minSteps(int N){
    vector<int> dp(N+1, 0);
    for(int n=2; n<=N; n++){
        dp[n] = dp[n-1] + 1;
        for(int a=2; (long long)a*a<=n; a++){
            if(n % a == 0){
                int b = n / a;          // b >= a, so max(a,b)=b
                dp[n] = min(dp[n], dp[b] + 1);
            }
        }
    }
    return dp[N];
}

int main(){
    cout << minSteps(100) << "\n";
    return 0;
}
