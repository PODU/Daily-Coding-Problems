// Day 156: Min perfect squares summing to n via DP. dp[i] = min over squares
// j*j<=i of dp[i-j*j]+1. Time O(n*sqrt(n)), Space O(n).
#include <bits/stdc++.h>
using namespace std;

int numSquares(int n) {
    vector<int> dp(n + 1, INT_MAX);
    dp[0] = 0;
    for (int i = 1; i <= n; i++)
        for (int j = 1; j * j <= i; j++)
            dp[i] = min(dp[i], dp[i - j * j] + 1);
    return dp[n];
}

int main() {
    cout << numSquares(13) << "\n"; // 2
    cout << numSquares(27) << "\n"; // 3
    return 0;
}
