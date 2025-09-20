// Day 304: Probability knight stays on board after k random moves. DP over board.
// Time O(k*N^2*8), space O(N^2).
#include <bits/stdc++.h>
using namespace std;
double knightProb(int N, int k, int sr, int sc) {
    vector<vector<double>> dp(N, vector<double>(N, 0));
    dp[sr][sc] = 1.0;
    int dr[] = {1,1,-1,-1,2,2,-2,-2}, dc[] = {2,-2,2,-2,1,-1,1,-1};
    for (int step = 0; step < k; step++) {
        vector<vector<double>> nd(N, vector<double>(N, 0));
        for (int r = 0; r < N; r++) for (int c = 0; c < N; c++) if (dp[r][c] > 0)
            for (int d = 0; d < 8; d++) {
                int nr = r + dr[d], nc = c + dc[d];
                if (nr >= 0 && nr < N && nc >= 0 && nc < N) nd[nr][nc] += dp[r][c] / 8.0;
            }
        dp = nd;
    }
    double tot = 0;
    for (auto& row : dp) for (double v : row) tot += v;
    return tot;
}
int main() {
    cout << knightProb(8, 1, 0, 0) << "\n"; // 0.25 (corner: 2 of 8 moves stay on board)
}
