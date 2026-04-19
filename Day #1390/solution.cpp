// Max coins on a grid moving right/down via DP: dp[i][j]=m[i][j]+max(up,left). O(R*C) time/space.
#include <iostream>
#include <vector>
#include <algorithm>
using namespace std;

int main() {
    vector<vector<int>> m = {
        {0, 3, 1, 1},
        {2, 0, 0, 4},
        {1, 5, 3, 1}
    };
    int R = m.size(), C = m[0].size();
    vector<vector<int>> dp(R, vector<int>(C, 0));
    for (int i = 0; i < R; ++i)
        for (int j = 0; j < C; ++j) {
            int best = 0;
            if (i > 0) best = max(best, dp[i-1][j]);
            if (j > 0) best = max(best, dp[i][j-1]);
            dp[i][j] = m[i][j] + ((i == 0 && j == 0) ? 0 : best);
        }
    cout << dp[R-1][C-1] << "\n"; // 12
    return 0;
}
