// Day 391: Longest common contiguous subsequence (substring) of two histories.
// DP table lcs[i][j] = length of common run ending at user1[i-1], user2[j-1].
// Time O(n*m), Space O(n*m).
#include <bits/stdc++.h>
using namespace std;

vector<string> longestCommon(const vector<string>& a, const vector<string>& b) {
    int n = a.size(), m = b.size();
    vector<vector<int>> dp(n + 1, vector<int>(m + 1, 0));
    int best = 0, endI = 0;
    for (int i = 1; i <= n; ++i)
        for (int j = 1; j <= m; ++j)
            if (a[i - 1] == b[j - 1]) {
                dp[i][j] = dp[i - 1][j - 1] + 1;
                if (dp[i][j] > best) { best = dp[i][j]; endI = i; }
            }
    return vector<string>(a.begin() + endI - best, a.begin() + endI);
}

int main() {
    vector<string> u1 = {"/home", "/register", "/login", "/user", "/one", "/two"};
    vector<string> u2 = {"/home", "/red", "/login", "/user", "/one", "/pink"};
    auto res = longestCommon(u1, u2);
    cout << "[";
    for (size_t i = 0; i < res.size(); ++i)
        cout << "'" << res[i] << "'" << (i + 1 < res.size() ? ", " : "");
    cout << "]" << endl;
    return 0;
}
