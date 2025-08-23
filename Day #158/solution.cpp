// Day 158: Count paths (right/down only) avoiding walls. DP: dp[i][j] = ways
// into a free cell from top+left; walls contribute 0. Time O(N*M), Space O(M).
#include <bits/stdc++.h>
using namespace std;

long long countPaths(const vector<vector<int>>& grid) {
    int n = grid.size(), m = grid[0].size();
    vector<long long> dp(m, 0);
    dp[0] = grid[0][0] == 0 ? 1 : 0;
    for (int i = 0; i < n; i++) {
        for (int j = 0; j < m; j++) {
            if (grid[i][j] == 1) { dp[j] = 0; continue; }
            if (j > 0) dp[j] += dp[j - 1];
        }
    }
    return dp[m - 1];
}

int main() {
    vector<vector<int>> grid = {
        {0, 0, 1},
        {0, 0, 1},
        {1, 0, 0}
    };
    cout << countPaths(grid) << "\n"; // 2
    return 0;
}
