// Day 1299: Count ordered ways to climb N stairs using step sizes from set X.
// DP: dp[i] = sum(dp[i-x]) for x in X. O(N*|X|) time, O(N) space.
#include <bits/stdc++.h>
using namespace std;

long long climbWays(int n, const vector<int>& steps) {
    vector<long long> dp(n + 1, 0);
    dp[0] = 1;
    for (int i = 1; i <= n; i++)
        for (int x : steps)
            if (i - x >= 0) dp[i] += dp[i - x];
    return dp[n];
}

int main() {
    cout << climbWays(4, {1, 2}) << endl;    // 5
    cout << climbWays(10, {1, 3, 5}) << endl; // generalized example
    return 0;
}
