// Day 1305: Longest common subsequence of three strings.
// 3D DP over prefixes. O(|a||b||c|) time, O(|a||b||c|) space.
#include <bits/stdc++.h>
using namespace std;

int lcs3(const string& a, const string& b, const string& c) {
    int n = a.size(), m = b.size(), p = c.size();
    vector<vector<vector<int>>> dp(n + 1,
        vector<vector<int>>(m + 1, vector<int>(p + 1, 0)));
    for (int i = 1; i <= n; i++)
        for (int j = 1; j <= m; j++)
            for (int k = 1; k <= p; k++) {
                if (a[i - 1] == b[j - 1] && b[j - 1] == c[k - 1])
                    dp[i][j][k] = dp[i - 1][j - 1][k - 1] + 1;
                else
                    dp[i][j][k] = max({dp[i - 1][j][k], dp[i][j - 1][k], dp[i][j][k - 1]});
            }
    return dp[n][m][p];
}

int main() {
    cout << lcs3("epidemiologist", "refrigeration",
                 "supercalifragilisticexpialodocious") << endl; // 5
    return 0;
}
