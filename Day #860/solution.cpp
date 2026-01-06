// Day 860: Regex matching with '.' and '*'.
// Approach: bottom-up DP, dp[i][j] = does s[i:] match p[j:].
// Time: O(n*m), Space: O(n*m).
#include <bits/stdc++.h>
using namespace std;

bool isMatch(const string& s, const string& p) {
    int n = s.size(), m = p.size();
    vector<vector<bool>> dp(n + 1, vector<bool>(m + 1, false));
    dp[n][m] = true;
    for (int i = n; i >= 0; i--)
        for (int j = m - 1; j >= 0; j--) {
            bool first = i < n && (p[j] == s[i] || p[j] == '.');
            if (j + 1 < m && p[j + 1] == '*')
                dp[i][j] = dp[i][j + 2] || (first && dp[i + 1][j]);
            else
                dp[i][j] = first && dp[i + 1][j + 1];
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
