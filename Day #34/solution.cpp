// Min-insertion palindrome, lexicographically earliest. DP over substrings; O(n^2) states, O(n^3) overall.
#include <bits/stdc++.h>
using namespace std;

string solve(const string& s) {
    int n = s.size();
    if (n == 0) return "";
    vector<vector<string>> dp(n, vector<string>(n));
    for (int i = 0; i < n; i++) dp[i][i] = string(1, s[i]);
    for (int len = 2; len <= n; len++) {
        for (int i = 0; i + len - 1 < n; i++) {
            int j = i + len - 1;
            if (s[i] == s[j]) {
                string inner = (i + 1 <= j - 1) ? dp[i + 1][j - 1] : "";
                dp[i][j] = s[i] + inner + s[j];
            } else {
                string c1 = s[i] + dp[i + 1][j] + s[i];
                string c2 = s[j] + dp[i][j - 1] + s[j];
                if (c1.size() < c2.size()) dp[i][j] = c1;
                else if (c2.size() < c1.size()) dp[i][j] = c2;
                else dp[i][j] = min(c1, c2);
            }
        }
    }
    return dp[0][n - 1];
}

int main() {
    cout << "\"" << solve("race") << "\"" << "\n";
    return 0;
}
