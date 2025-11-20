// Day 635: Regular expression matching with '.' and '*' (full-string match).
// Approach: bottom-up DP; dp[i][j] = does s[i:] match p[j:].
// Time: O(m*n), Space: O(m*n).
#include <bits/stdc++.h>
using namespace std;

bool isMatch(const string& s, const string& p) {
    int m = s.size(), n = p.size();
    vector<vector<bool>> dp(m + 1, vector<bool>(n + 1, false));
    dp[m][n] = true;
    for (int i = m; i >= 0; i--) {
        for (int j = n - 1; j >= 0; j--) {
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
    cout << boolalpha;
    cout << isMatch("ray", "ra.") << "\n";      // true
    cout << isMatch("raymond", "ra.") << "\n";  // false
    cout << isMatch("chat", ".*at") << "\n";    // true
    cout << isMatch("chats", ".*at") << "\n";   // false
    return 0;
}
