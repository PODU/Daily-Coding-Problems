// Day 1455: Maximum top-to-bottom path sum in a triangle. Bottom-up DP folding
// each row into the one above. Time O(n^2), Space O(n).
#include <bits/stdc++.h>
using namespace std;

int maxPathSum(vector<vector<int>> triangle) {
    if (triangle.empty()) return 0;
    vector<int> dp = triangle.back();
    for (int r = (int)triangle.size() - 2; r >= 0; r--)
        for (int i = 0; i <= r; i++)
            dp[i] = triangle[r][i] + max(dp[i], dp[i + 1]);
    return dp[0];
}

int main() {
    vector<vector<int>> triangle = {{1}, {2, 3}, {1, 5, 1}};
    cout << maxPathSum(triangle) << "\n"; // 9  (1 -> 3 -> 5)
    return 0;
}
