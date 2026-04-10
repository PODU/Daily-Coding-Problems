// Day 1333: Count right/down paths from top-left to bottom-right avoiding walls (1).
// DP: dp[i][j] = dp[i-1][j] + dp[i][j-1], 0 at walls. O(N*M) time, O(M) space.
#include <bits/stdc++.h>
using namespace std;

long long countPaths(const vector<vector<int>>& g) {
    int n = g.size(), m = g[0].size();
    vector<long long> dp(m, 0);
    for (int i = 0; i < n; i++)
        for (int j = 0; j < m; j++) {
            if (g[i][j] == 1) { dp[j] = 0; continue; }
            if (i == 0 && j == 0) dp[j] = 1;
            else dp[j] = (i ? dp[j] : 0) + (j ? dp[j - 1] : 0);
        }
    return dp[m - 1];
}

int main() {
    vector<vector<int>> g = {{0,0,1},{0,0,1},{1,0,0}};
    cout << countPaths(g) << endl; // 2
    return 0;
}
