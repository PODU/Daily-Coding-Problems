// Day 867: Maximum weight path from top to bottom of a triangle.
// Approach: bottom-up DP, fold each row into the one above using max of adjacent.
// Time: O(n^2), Space: O(n).
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
    cout << maxPath(t) << endl; // 9
    return 0;
}
