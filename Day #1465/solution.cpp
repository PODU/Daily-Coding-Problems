// Optimal coin game via interval DP. dp[i][j] = best score first player can guarantee on coins[i..j].
// Time O(n^2), Space O(n^2).
#include <bits/stdc++.h>
using namespace std;

int coinGame(const vector<int>& v) {
    int n = v.size();
    if (n == 0) return 0;
    vector<vector<int>> dp(n, vector<int>(n, 0));
    for (int len = 1; len <= n; ++len) {
        for (int i = 0; i + len - 1 < n; ++i) {
            int j = i + len - 1;
            int a = (i + 2 <= j) ? dp[i + 2][j] : 0;           // opponent takes from i+1
            int b = (i + 1 <= j - 1) ? dp[i + 1][j - 1] : 0;   // opponent takes opposite end
            int c = (i <= j - 2) ? dp[i][j - 2] : 0;
            int takeFirst = v[i] + min(a, b);
            int takeLast  = v[j] + min(b, c);
            dp[i][j] = max(takeFirst, takeLast);
        }
    }
    return dp[0][n - 1];
}

int main() {
    cout << coinGame({8, 15, 3, 7}) << "\n";
    cout << coinGame({2, 2, 2, 2}) << "\n";
    return 0;
}
