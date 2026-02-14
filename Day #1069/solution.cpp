// Coin game DP: dp[i][j] = max guaranteed for current player on coins[i..j]. O(n^2) time/space.
#include <bits/stdc++.h>
using namespace std;

int coinGame(vector<int>& v) {
    int n = v.size();
    if (n == 0) return 0;
    vector<vector<int>> dp(n, vector<int>(n, 0));
    for (int i = 0; i < n; i++) dp[i][i] = v[i];
    for (int len = 2; len <= n; len++) {
        for (int i = 0; i <= n - len; i++) {
            int j = i + len - 1;
            int takeI = v[i] + min(
                (i+2 <= j ? dp[i+2][j] : 0),
                (i+1 <= j-1 ? dp[i+1][j-1] : 0)
            );
            int takeJ = v[j] + min(
                (i+1 <= j-1 ? dp[i+1][j-1] : 0),
                (i <= j-2 ? dp[i][j-2] : 0)
            );
            dp[i][j] = max(takeI, takeJ);
        }
    }
    return dp[0][n-1];
}

int main() {
    vector<int> a = {8, 15, 3, 7};
    cout << "Max guaranteed: " << coinGame(a) << "\n";
    vector<int> b = {2, 2, 2, 2};
    cout << "Max guaranteed: " << coinGame(b) << "\n";
    return 0;
}
