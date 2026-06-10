// Day 1636: Can a string be made a palindrome by deleting at most k chars.
// min deletions = n - LongestPalindromicSubsequence; DP O(n^2) time/space.
#include <bits/stdc++.h>
using namespace std;

bool canMakePalindrome(const string& s, int k) {
    int n = s.size();
    if (n == 0) return true;
    vector<vector<int>> dp(n, vector<int>(n, 0));
    for (int i = 0; i < n; i++) dp[i][i] = 1;
    for (int len = 2; len <= n; len++)
        for (int i = 0; i + len - 1 < n; i++) {
            int j = i + len - 1;
            if (s[i] == s[j]) dp[i][j] = 2 + (len > 2 ? dp[i + 1][j - 1] : 0);
            else dp[i][j] = max(dp[i + 1][j], dp[i][j - 1]);
        }
    int lps = dp[0][n - 1];
    return (n - lps) <= k;
}

int main() {
    string s = "waterrfetawx";
    int k = 2;
    cout << (canMakePalindrome(s, k) ? "True" : "False") << endl;
    return 0;
}
