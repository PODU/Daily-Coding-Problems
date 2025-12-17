// Day 754: Optimal coin game (interval DP / minimax).
// dp[i][j] = max value first player guarantees from coins[i..j].
// Time: O(n^2), Space: O(n^2).
#include <bits/stdc++.h>
using namespace std;

long long maxCoins(const vector<int>& v) {
    int n = v.size();
    if (n == 0) return 0;
    vector<vector<long long>> dp(n, vector<long long>(n, 0));
    vector<long long> pre(n + 1, 0);
    for (int i = 0; i < n; ++i) pre[i + 1] = pre[i] + v[i];
    auto sum = [&](int i, int j) { return pre[j + 1] - pre[i]; };

    for (int i = 0; i < n; ++i) dp[i][i] = v[i];
    for (int len = 2; len <= n; ++len)
        for (int i = 0; i + len - 1 < n; ++i) {
            int j = i + len - 1;
            long long takeLeft  = v[i] + sum(i + 1, j) - dp[i + 1][j];
            long long takeRight = v[j] + sum(i, j - 1) - dp[i][j - 1];
            dp[i][j] = max(takeLeft, takeRight);
        }
    return dp[0][n - 1];
}

int main() {
    vector<int> coins = {8, 15, 3, 7};
    cout << maxCoins(coins) << "\n";   // 22
    return 0;
}
