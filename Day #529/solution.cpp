// Day 529: Split a string into the fewest palindromic substrings.
// DP: isPal[i][j] in O(n^2); dp[i] = min cuts for prefix i with parent pointers
// to reconstruct one optimal partition. Time O(n^2), space O(n^2).
#include <bits/stdc++.h>
using namespace std;

vector<string> minPalindromePartition(const string &s) {
    int n = s.size();
    vector<vector<bool>> pal(n, vector<bool>(n, false));
    for (int i = n - 1; i >= 0; i--)
        for (int j = i; j < n; j++)
            pal[i][j] = (s[i] == s[j]) && (j - i < 2 || pal[i + 1][j - 1]);

    vector<int> dp(n + 1, INT_MAX), prev(n + 1, -1);
    dp[0] = 0;
    for (int j = 1; j <= n; j++)
        for (int i = 0; i < j; i++)
            if (pal[i][j - 1] && dp[i] != INT_MAX && dp[i] + 1 < dp[j]) {
                dp[j] = dp[i] + 1;
                prev[j] = i;
            }

    vector<string> parts;
    for (int j = n; j > 0; j = prev[j]) parts.push_back(s.substr(prev[j], j - prev[j]));
    reverse(parts.begin(), parts.end());
    return parts;
}

int main() {
    string s = "racecarannakayak";
    vector<string> parts = minPalindromePartition(s);
    cout << "[";
    for (size_t i = 0; i < parts.size(); i++)
        cout << "\"" << parts[i] << "\"" << (i + 1 < parts.size() ? ", " : "");
    cout << "]\n"; // expected: ["racecar", "anna", "kayak"]
    return 0;
}
