// Subset-sum DP with reconstruction. O(n*k) time, O(n*k) space. Result sorted desc.
#include <bits/stdc++.h>
using namespace std;

// Returns {found, subset}. subset sums to k.
pair<bool, vector<int>> subsetSum(const vector<int>& s, int k) {
    int n = s.size();
    vector<vector<char>> dp(n + 1, vector<char>(k + 1, 0));
    for (int i = 0; i <= n; i++) dp[i][0] = 1;
    for (int i = 1; i <= n; i++)
        for (int j = 0; j <= k; j++) {
            dp[i][j] = dp[i - 1][j];
            if (j >= s[i - 1] && dp[i - 1][j - s[i - 1]]) dp[i][j] = 1;
        }
    if (!dp[n][k]) return {false, {}};
    vector<int> res;
    int j = k;
    for (int i = n; i >= 1; i--) {
        if (!dp[i - 1][j]) { res.push_back(s[i - 1]); j -= s[i - 1]; }
    }
    sort(res.rbegin(), res.rend());
    return {true, res};
}

void printRes(const pair<bool, vector<int>>& r) {
    if (!r.first) { cout << "null\n"; return; }
    cout << "[";
    for (size_t i = 0; i < r.second.size(); i++) { if (i) cout << ", "; cout << r.second[i]; }
    cout << "]\n";
}

int main() {
    vector<int> s = {12, 1, 61, 5, 9, 2};
    printRes(subsetSum(s, 24));
    printRes(subsetSum(s, 1000));
    return 0;
}
