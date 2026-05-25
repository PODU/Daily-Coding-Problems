// Grid DP: max coins from top-left to bottom-right moving right/down only.
// dp[j] = grid + max(top, left). Time O(m*n), Space O(n).
#include <iostream>
#include <vector>
#include <algorithm>
using namespace std;

int main() {
    vector<vector<int>> grid = {
        {0, 3, 1, 1},
        {2, 0, 0, 4},
        {1, 5, 3, 1}
    };
    int m = grid.size(), n = grid[0].size();
    vector<int> dp(n, 0);
    for (int i = 0; i < m; ++i) {
        for (int j = 0; j < n; ++j) {
            int best = 0;
            if (i == 0 && j == 0) best = 0;
            else if (i == 0) best = dp[j - 1];
            else if (j == 0) best = dp[j];
            else best = max(dp[j], dp[j - 1]);
            dp[j] = grid[i][j] + best;
        }
    }
    cout << dp[n - 1] << '\n';
    return 0;
}
