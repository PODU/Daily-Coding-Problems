// Day 1215: Min steps to reduce N to 1 (decrement, or replace by larger factor).
// DP: dp[i] = 1 + min(dp[i-1], dp[i/d] for divisors d). Time O(N sqrt N), Space O(N).
#include <bits/stdc++.h>
using namespace std;

int minSteps(int n) {
    vector<int> dp(n + 1, INT_MAX);
    dp[1] = 0;
    for (int i = 2; i <= n; i++) {
        dp[i] = dp[i - 1] + 1;                    // decrement
        for (int d = 2; (long long)d * d <= i; d++)
            if (i % d == 0) dp[i] = min(dp[i], dp[i / d] + 1); // larger factor i/d
    }
    return dp[n];
}

int main() {
    cout << minSteps(100) << "\n"; // 5  (100 -> 10 -> 9 -> 3 -> 2 -> 1)
    return 0;
}
