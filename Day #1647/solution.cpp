// Coins-in-a-row: dp[i][j] = max value first-to-move guarantees from coins[i..j],
// dp[i][j]=max(v[i]+min(dp[i+2][j],dp[i+1][j-1]), v[j]+min(dp[i+1][j-1],dp[i][j-2])). Time/Space O(n^2).
#include <iostream>
#include <vector>
using namespace std;

int maxGuaranteed(const vector<int>& v) {
    int n = (int)v.size();
    if (n == 0) return 0;
    vector<vector<int>> dp(n, vector<int>(n, 0));
    for (int len = 1; len <= n; len++) {
        for (int i = 0; i + len - 1 < n; i++) {
            int j = i + len - 1;
            int a = (i + 2 <= j) ? dp[i + 2][j] : 0;
            int b = (i + 1 <= j - 1) ? dp[i + 1][j - 1] : 0;
            int c = (i <= j - 2) ? dp[i][j - 2] : 0;
            int takeFirst = v[i] + min(a, b);
            int takeLast = v[j] + min(b, c);
            dp[i][j] = max(takeFirst, takeLast);
        }
    }
    return dp[0][n - 1];
}

int main() {
    vector<int> coins = {3, 9, 1, 2};
    cout << maxGuaranteed(coins) << endl;
    return 0;
}
