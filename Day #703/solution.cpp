// Day 703: Probability a knight stays on an 8x8 board after k random moves.
// Approach: DP over board cells; each move spreads prob/8 to valid targets.
// Time O(k * N^2 * 8), Space O(N^2).
#include <bits/stdc++.h>
using namespace std;

double knightProbability(int N, int k, int r, int c) {
    int dr[] = {1,1,-1,-1,2,2,-2,-2}, dc[] = {2,-2,2,-2,1,-1,1,-1};
    vector<vector<double>> dp(N, vector<double>(N, 0));
    dp[r][c] = 1.0;
    for (int step = 0; step < k; ++step) {
        vector<vector<double>> nd(N, vector<double>(N, 0));
        for (int i = 0; i < N; ++i)
            for (int j = 0; j < N; ++j)
                if (dp[i][j] > 0)
                    for (int m = 0; m < 8; ++m) {
                        int ni = i + dr[m], nj = j + dc[m];
                        if (ni >= 0 && ni < N && nj >= 0 && nj < N)
                            nd[ni][nj] += dp[i][j] / 8.0;
                    }
        dp = nd;
    }
    double total = 0;
    for (auto& row : dp) for (double v : row) total += v;
    return total;
}

int main() {
    cout << knightProbability(8, 2, 0, 0) << "\n"; // 0.1875
    return 0;
}
