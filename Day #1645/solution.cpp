// Regex full-match with '.' and '*' via DP: dp[i][j] = s[0..i) matches p[0..j).
// '*' uses zero-copy (dp[i][j-2]) or one-more (prev char match). Time/Space O(m*n).
#include <iostream>
#include <string>
#include <vector>
using namespace std;

bool isMatch(const string& s, const string& p) {
    int m = (int)s.size(), n = (int)p.size();
    vector<vector<bool>> dp(m + 1, vector<bool>(n + 1, false));
    dp[0][0] = true;
    for (int j = 1; j <= n; ++j)
        if (p[j - 1] == '*' && j >= 2) dp[0][j] = dp[0][j - 2];
    for (int i = 1; i <= m; ++i) {
        for (int j = 1; j <= n; ++j) {
            if (p[j - 1] == '*') {
                dp[i][j] = (j >= 2 && dp[i][j - 2]);
                if (j >= 2 && (p[j - 2] == '.' || p[j - 2] == s[i - 1]))
                    dp[i][j] = dp[i][j] || dp[i - 1][j];
            } else if (p[j - 1] == '.' || p[j - 1] == s[i - 1]) {
                dp[i][j] = dp[i - 1][j - 1];
            }
        }
    }
    return dp[m][n];
}

int main() {
    cout << (isMatch("ray", "ra.") ? "true" : "false") << "\n";
    cout << (isMatch("raymond", "ra.") ? "true" : "false") << "\n";
    cout << (isMatch("chat", ".*at") ? "true" : "false") << "\n";
    cout << (isMatch("chats", ".*at") ? "true" : "false") << "\n";
    return 0;
}
