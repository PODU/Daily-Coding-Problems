// Can form palindrome by deleting <= k chars: min deletions = n - LPS(s).
// LPS via DP. Time O(n^2), Space O(n^2).
#include <bits/stdc++.h>
using namespace std;

int lps(const string& s) {
    int n = s.size();
    vector<vector<int>> dp(n, vector<int>(n, 0));
    for (int i = 0; i < n; i++) dp[i][i] = 1;
    for (int len = 2; len <= n; len++)
        for (int i = 0; i + len - 1 < n; i++) {
            int j = i + len - 1;
            if (s[i] == s[j]) dp[i][j] = (len == 2 ? 2 : dp[i + 1][j - 1] + 2);
            else dp[i][j] = max(dp[i + 1][j], dp[i][j - 1]);
        }
    return dp[0][n - 1];
}

bool canMakePalindrome(const string& s, int k) {
    return (int)s.size() - lps(s) <= k;
}

int main() {
    cout << (canMakePalindrome("waterrfetawx", 2) ? "True" : "False") << "\n";
    return 0;
}
