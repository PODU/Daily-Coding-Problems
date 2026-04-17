// Subset sum returning an actual subset via DP + backtracking reconstruction.
// Time O(n*k), Space O(n*k). Returns empty/"null" if no subset sums to k.
#include <bits/stdc++.h>
using namespace std;

vector<int> subsetSum(const vector<int>& s, int k, bool& found) {
    int n = s.size();
    vector<vector<char>> dp(n + 1, vector<char>(k + 1, 0));
    for (int i = 0; i <= n; i++) dp[i][0] = 1;
    for (int i = 1; i <= n; i++)
        for (int j = 0; j <= k; j++) {
            dp[i][j] = dp[i - 1][j];
            if (j >= s[i - 1] && dp[i - 1][j - s[i - 1]]) dp[i][j] = 1;
        }
    found = dp[n][k];
    vector<int> res;
    if (!found) return res;
    int j = k;
    for (int i = n; i >= 1; i--) {
        if (!dp[i - 1][j]) { res.push_back(s[i - 1]); j -= s[i - 1]; }
    }
    reverse(res.begin(), res.end());
    return res;
}

int main() {
    bool found;
    auto res = subsetSum({12, 1, 61, 5, 9, 2}, 24, found);
    if (!found) { cout << "null\n"; return 0; }
    cout << "[";
    for (size_t i = 0; i < res.size(); i++) { cout << res[i]; if (i + 1 < res.size()) cout << ", "; }
    cout << "]\n";
    return 0;
}
