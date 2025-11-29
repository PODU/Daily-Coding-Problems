// Day 672: Maximum-weight top-to-bottom triangle path. Bottom-up DP folding rows.
// Time O(n^2) cells, Space O(n).
#include <bits/stdc++.h>
using namespace std;

int maxPath(vector<vector<int>> t) {
    vector<int> dp = t.back();
    for (int i = (int)t.size() - 2; i >= 0; i--)
        for (int j = 0; j <= i; j++)
            dp[j] = t[i][j] + max(dp[j], dp[j + 1]);
    return dp[0];
}

int main() {
    vector<vector<int>> t = {{1}, {2, 3}, {1, 5, 1}};
    cout << maxPath(t) << "\n"; // 9
    return 0;
}
