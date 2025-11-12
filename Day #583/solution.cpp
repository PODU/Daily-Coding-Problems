// Grid DP: dp[i][j] = grid[i][j] + max(dp[i-1][j], dp[i][j-1]). Time O(R*C), Space O(R*C).
#include <iostream>
#include <vector>
#include <algorithm>
#include <cassert>
using namespace std;

int maxCoins(const vector<vector<int>>& grid) {
    int R = grid.size(), C = grid[0].size();
    vector<vector<int>> dp(R, vector<int>(C, 0));
    for (int i = 0; i < R; ++i)
        for (int j = 0; j < C; ++j) {
            int best = 0;
            if (i > 0) best = max(best, dp[i-1][j]);
            if (j > 0) best = max(best, dp[i][j-1]);
            dp[i][j] = grid[i][j] + ((i == 0 && j == 0) ? 0 : best);
        }
    return dp[R-1][C-1];
}

int main() {
    vector<vector<int>> grid = {{0,3,1,1},{2,0,0,4},{1,5,3,1}};
    int result = maxCoins(grid);
    assert(result == 12);
    cout << "The most we can collect is 0 + 2 + 1 + 5 + 3 + 1 = " << result << " coins." << endl;
    return 0;
}
