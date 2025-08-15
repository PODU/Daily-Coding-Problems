// Day 122: Max coins from top-left to bottom-right moving right/down.
// DP O(R*C) time and space, with path reconstruction (prefer left on ties).
#include <bits/stdc++.h>
using namespace std;

int main() {
    vector<vector<int>> g = {{0, 3, 1, 1}, {2, 0, 0, 4}, {1, 5, 3, 1}};
    int R = g.size(), C = g[0].size();
    vector<vector<int>> dp(R, vector<int>(C, 0));
    for (int i = 0; i < R; i++)
        for (int j = 0; j < C; j++) {
            int best = 0;
            if (i == 0 && j == 0) best = 0;
            else if (i == 0) best = dp[i][j - 1];
            else if (j == 0) best = dp[i - 1][j];
            else best = max(dp[i - 1][j], dp[i][j - 1]);
            dp[i][j] = g[i][j] + best;
        }
    vector<int> path;
    int i = R - 1, j = C - 1;
    while (i > 0 || j > 0) {
        path.push_back(g[i][j]);
        if (i == 0) j--;
        else if (j == 0) i--;
        else if (dp[i - 1][j] > dp[i][j - 1]) i--;
        else j--;
    }
    path.push_back(g[0][0]);
    reverse(path.begin(), path.end());
    string s;
    for (size_t t = 0; t < path.size(); t++) {
        if (t) s += " + ";
        s += to_string(path[t]);
    }
    cout << "The most we can collect is " << s << " = " << dp[R - 1][C - 1] << " coins." << endl;
    return 0;
}
