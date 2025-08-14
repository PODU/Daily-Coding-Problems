// Day 121: Can form palindrome by deleting at most k chars.
// Min deletions = n - LongestPalindromicSubsequence. DP O(n^2) time, O(n^2) space.
#include <bits/stdc++.h>
using namespace std;

int lps(const string& s) {
    int n = s.size();
    if (n == 0) return 0;
    vector<vector<int>> dp(n, vector<int>(n, 0));
    for (int i = 0; i < n; i++) dp[i][i] = 1;
    for (int len = 2; len <= n; len++)
        for (int i = 0; i + len - 1 < n; i++) {
            int j = i + len - 1;
            if (s[i] == s[j]) dp[i][j] = 2 + (len == 2 ? 0 : dp[i + 1][j - 1]);
            else dp[i][j] = max(dp[i + 1][j], dp[i][j - 1]);
        }
    return dp[0][n - 1];
}

bool canMakePalindrome(const string& s, int k) {
    return (int)s.size() - lps(s) <= k;
}

int main() {
    string s = "waterrfetawx";
    int k = 2;
    if (canMakePalindrome(s, k))
        cout << "You could delete f and x to get 'waterretaw'." << endl;
    else
        cout << "Not possible" << endl;
    return 0;
}
