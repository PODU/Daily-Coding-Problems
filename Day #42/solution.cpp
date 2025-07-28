// Subset Sum: boolean DP dp[i][j] = can make j with first i items, then backtrack.
// Found subset is sorted descending for a deterministic [12, 9, 2, 1] output.
// Time O(n*k), Space O(n*k).
#include <bits/stdc++.h>
using namespace std;

bool subsetSum(const vector<int>& S, int k, vector<int>& res) {
    int n = (int)S.size();
    vector<vector<char>> dp(n + 1, vector<char>(k + 1, 0));
    for (int i = 0; i <= n; i++) dp[i][0] = 1;
    for (int i = 1; i <= n; i++)
        for (int j = 1; j <= k; j++) {
            dp[i][j] = dp[i - 1][j];
            if (j >= S[i - 1] && dp[i - 1][j - S[i - 1]]) dp[i][j] = 1;
        }
    if (!dp[n][k]) return false;
    int j = k;
    for (int i = n; i >= 1; i--) {
        if (dp[i - 1][j]) continue;          // item i-1 not needed
        res.push_back(S[i - 1]);              // item i-1 must be included
        j -= S[i - 1];
    }
    sort(res.rbegin(), res.rend());
    return true;
}

string fmt(const vector<int>& v) {
    string s = "[";
    for (size_t i = 0; i < v.size(); i++) {
        s += to_string(v[i]);
        if (i + 1 < v.size()) s += ", ";
    }
    return s + "]";
}

int main() {
    vector<int> S = {12, 1, 61, 5, 9, 2};
    int k = 24;
    vector<int> res;
    if (subsetSum(S, k, res)) cout << fmt(res) << "\n";
    else cout << "null\n";
    return 0;
}
