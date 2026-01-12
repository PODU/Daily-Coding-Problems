// Min steps to 1: dp[i] = 1 + min(dp[i-1], dp[i/a] over factors a). Time O(N*sqrt N), Space O(N).
#include <bits/stdc++.h>
using namespace std;

int minSteps(int n){
    vector<int> dp(n + 1, 0);
    for(int i = 2; i <= n; i++){
        dp[i] = dp[i - 1] + 1; // decrement step
        for(int a = 2; (long long)a * a <= i; a++){
            if(i % a == 0){
                int larger = i / a; // larger of the two factors
                dp[i] = min(dp[i], 1 + dp[larger]);
            }
        }
    }
    return dp[n];
}

int main(){
    cout << minSteps(100) << "\n";
    return 0;
}
