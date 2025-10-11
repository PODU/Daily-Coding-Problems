// Day 411: Can we make s a palindrome by deleting at most k chars?
// Answer: n - LPS(s) <= k, where LPS = longest palindromic subsequence via DP. O(n^2) time, O(n^2) space.
#include <bits/stdc++.h>
using namespace std;

int lps(const string& s) {
    int n = s.size();
    vector<vector<int>> dp(n, vector<int>(n, 0));
    for (int i = n - 1; i >= 0; i--) {
        dp[i][i] = 1;
        for (int j = i + 1; j < n; j++) {
            if (s[i] == s[j]) dp[i][j] = dp[i + 1][j - 1] + 2;
            else dp[i][j] = max(dp[i + 1][j], dp[i][j - 1]);
        }
    }
    return dp[0][n - 1];
}

bool canMakePalindrome(const string& s, int k) {
    return (int)s.size() - lps(s) <= k;
}

int main() {
    string s = "waterrfetawx";
    int k = 2;
    cout << (canMakePalindrome(s, k) ? "True" : "False") << endl;
    return 0;
}
