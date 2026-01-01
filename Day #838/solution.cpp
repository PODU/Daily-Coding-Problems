// Day 838: Max coins moving only right/down through a grid.
// DP: dp[i][j] = grid[i][j] + max(dp[i-1][j], dp[i][j-1]).  Time O(R*C), Space O(R*C).
#include <bits/stdc++.h>
using namespace std;

int maxCoins(const vector<vector<int>>& grid) {
    if (grid.empty() || grid[0].empty()) return 0;
    int rows = grid.size(), cols = grid[0].size();
    vector<vector<int>> dp(rows, vector<int>(cols, 0));
    for (int i = 0; i < rows; ++i) {
        for (int j = 0; j < cols; ++j) {
            int best = 0;
            if (i > 0) best = max(best, dp[i - 1][j]);
            if (j > 0) best = max(best, dp[i][j - 1]);
            dp[i][j] = grid[i][j] + best;
        }
    }
    return dp[rows - 1][cols - 1];
}

int main() {
    vector<vector<int>> matrix = {
        {0, 3, 1, 1},
        {2, 0, 0, 4},
        {1, 5, 3, 1},
    };
    cout << maxCoins(matrix) << endl;
    return 0;
}
