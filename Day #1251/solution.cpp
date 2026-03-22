// Regex matching with '.' and '*' via DP; dp[i][j] = s[i:] matches p[j:]. O(m*n) time and space.
#include <bits/stdc++.h>
using namespace std;

bool isMatch(const string& s, const string& p) {
    int m = s.size(), n = p.size();
    vector<vector<bool>> dp(m + 1, vector<bool>(n + 1, false));
    dp[m][n] = true;
    for (int i = m; i >= 0; --i) {
        for (int j = n - 1; j >= 0; --j) {
            bool first = i < m && (p[j] == s[i] || p[j] == '.');
            if (j + 1 < n && p[j + 1] == '*') {
                dp[i][j] = dp[i][j + 2] || (first && dp[i + 1][j]);
            } else {
                dp[i][j] = first && dp[i + 1][j + 1];
            }
        }
    }
    return dp[0][0];
}

int main() {
    cout << (isMatch("ray", "ra.") ? "true" : "false") << "\n";
    cout << (isMatch("raymond", "ra.") ? "true" : "false") << "\n";
    cout << (isMatch("chat", ".*at") ? "true" : "false") << "\n";
    cout << (isMatch("chats", ".*at") ? "true" : "false") << "\n";
    return 0;
}
