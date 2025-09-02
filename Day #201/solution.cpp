// Day 201: Maximum weight path in a triangle.
// Bottom-up DP: each cell becomes its value + max of the two children below.
// Time: O(n^2), Space: O(n).
#include <bits/stdc++.h>
using namespace std;

int maxPath(vector<vector<int>> t) {
    vector<int> dp = t.back();
    for (int r = (int)t.size() - 2; r >= 0; --r)
        for (int c = 0; c <= r; ++c)
            dp[c] = t[r][c] + max(dp[c], dp[c + 1]);
    return dp[0];
}

int main() {
    cout << maxPath({{1}, {2, 3}, {1, 5, 1}}) << endl; // 9
    return 0;
}
