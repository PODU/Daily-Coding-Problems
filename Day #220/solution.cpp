// Day 220: Optimal coin-picking game (first player guaranteed max).
// Approach: interval DP. dp[i][j] = max you can collect from coins i..j vs optimal opponent.
// Time O(n^2), Space O(n^2).
#include <bits/stdc++.h>
using namespace std;

int maxCoins(const vector<int>& v) {
    int n = v.size();
    if (n == 0) return 0;
    vector<vector<int>> dp(n, vector<int>(n, 0));
    for (int i = 0; i < n; i++) dp[i][i] = v[i];
    for (int len = 2; len <= n; len++) {
        for (int i = 0; i + len - 1 < n; i++) {
            int j = i + len - 1;
            int takeI = v[i] + min(i + 2 <= j ? dp[i + 2][j] : 0,
                                   i + 1 <= j - 1 ? dp[i + 1][j - 1] : 0);
            int takeJ = v[j] + min(i + 1 <= j - 1 ? dp[i + 1][j - 1] : 0,
                                   i <= j - 2 ? dp[i][j - 2] : 0);
            dp[i][j] = max(takeI, takeJ);
        }
    }
    return dp[0][n - 1];
}

int main() {
    cout << maxCoins({8, 15, 3, 7}) << endl; // 22
    return 0;
}
