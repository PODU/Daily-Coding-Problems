// Regex matching with '.' and '*' via DP. Time O(m*n), Space O(m*n).
#include <bits/stdc++.h>
using namespace std;

bool isMatch(const string& s, const string& p) {
    int m = s.size(), n = p.size();
    vector<vector<bool>> dp(m + 1, vector<bool>(n + 1, false));
    dp[0][0] = true;
    for (int j = 1; j <= n; ++j)
        if (p[j - 1] == '*') dp[0][j] = dp[0][j - 2];
    for (int i = 1; i <= m; ++i) {
        for (int j = 1; j <= n; ++j) {
            if (p[j - 1] == '*') {
                dp[i][j] = dp[i][j - 2];
                if (p[j - 2] == '.' || p[j - 2] == s[i - 1])
                    dp[i][j] = dp[i][j] || dp[i - 1][j];
            } else if (p[j - 1] == '.' || p[j - 1] == s[i - 1]) {
                dp[i][j] = dp[i - 1][j - 1];
            }
        }
    }
    return dp[m][n];
}

int main() {
    // Sanity checks (internal)
    assert(isMatch("aa", "a*") == true);
    assert(isMatch("ab", ".*") == true);
    assert(isMatch("mississippi", "mis*is*p*.") == false);
    // README main example: regex = "ra.", string = "ray"
    cout << (isMatch("ray", "ra.") ? "true" : "false") << "\n";
    return 0;
}
