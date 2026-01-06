// Day 861: Probability a knight stays on an 8x8 board after k random moves.
// Approach: DP over moves; dp[r][c] = prob of being on this cell, spread 1/8 each move.
// Time: O(k * N^2), Space: O(N^2).
#include <bits/stdc++.h>
using namespace std;

double knightProbability(int N, int k, int sr, int sc) {
    int dr[] = {-2,-2,-1,-1,1,1,2,2};
    int dc[] = {-1,1,-2,2,-2,2,-1,1};
    vector<vector<double>> dp(N, vector<double>(N, 0.0));
    dp[sr][sc] = 1.0;
    for (int step = 0; step < k; step++) {
        vector<vector<double>> nx(N, vector<double>(N, 0.0));
        for (int r = 0; r < N; r++)
            for (int c = 0; c < N; c++)
                if (dp[r][c] > 0)
                    for (int d = 0; d < 8; d++) {
                        int nr = r + dr[d], nc = c + dc[d];
                        if (nr >= 0 && nr < N && nc >= 0 && nc < N)
                            nx[nr][nc] += dp[r][c] / 8.0;
                    }
        dp = nx;
    }
    double total = 0;
    for (auto& row : dp) for (double v : row) total += v;
    return total;
}

int main() {
    cout << knightProbability(8, 1, 0, 0) << endl;   // 0.25 (corner, 1 move)
    cout << knightProbability(8, 2, 0, 0) << endl;
    return 0;
}
