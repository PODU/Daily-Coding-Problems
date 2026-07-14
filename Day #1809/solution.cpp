// Knight-on-board probability after k random moves: DP over board states.
// dp[r][c] = prob of being on (r,c); each move spreads 1/8 to each target.
// Time: O(k*64*8). Space: O(64).
#include <bits/stdc++.h>
using namespace std;

double knightProbability(int N, int k, int r0, int c0) {
    int dr[] = {-2,-2,-1,-1,1,1,2,2};
    int dc[] = {-1,1,-2,2,-2,2,-1,1};
    vector<vector<double>> dp(N, vector<double>(N, 0.0));
    dp[r0][c0] = 1.0;
    for (int m = 0; m < k; m++) {
        vector<vector<double>> nxt(N, vector<double>(N, 0.0));
        for (int r = 0; r < N; r++)
            for (int c = 0; c < N; c++) {
                if (dp[r][c] == 0) continue;
                for (int d = 0; d < 8; d++) {
                    int nr = r + dr[d], nc = c + dc[d];
                    if (nr >= 0 && nr < N && nc >= 0 && nc < N)
                        nxt[nr][nc] += dp[r][c] / 8.0;
                }
            }
        dp = nxt;
    }
    double total = 0;
    for (auto& row : dp) for (double v : row) total += v;
    return total;
}

int main() {
    // Start at corner (0,0), k = 1: only 2 of 8 moves stay on board -> 0.25
    cout << knightProbability(8, 1, 0, 0) << "\n"; // 0.25
    return 0;
}
