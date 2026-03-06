// Knight on board probability via DP over moves. prob(m,r,c)=avg of 8 neighbors.
// Time: O(k*64*8), Space: O(64).
#include <bits/stdc++.h>
using namespace std;

double knightProbability(int k, int startR, int startC) {
    int dr[8] = {-2,-2,-1,-1,1,1,2,2};
    int dc[8] = {-1,1,-2,2,-2,2,-1,1};
    vector<vector<double>> prob(8, vector<double>(8, 1.0));
    for (int m = 1; m <= k; ++m) {
        vector<vector<double>> next(8, vector<double>(8, 0.0));
        for (int r = 0; r < 8; ++r)
            for (int c = 0; c < 8; ++c) {
                double s = 0.0;
                for (int d = 0; d < 8; ++d) {
                    int nr = r + dr[d], nc = c + dc[d];
                    if (nr >= 0 && nr < 8 && nc >= 0 && nc < 8)
                        s += prob[nr][nc];
                }
                next[r][c] = s / 8.0;
            }
        prob = next;
    }
    return prob[startR][startC];
}

int main() {
    double ans = knightProbability(1, 0, 0);
    ostringstream oss;
    oss << setprecision(2) << ans;
    cout << oss.str() << "\n";
    return 0;
}
