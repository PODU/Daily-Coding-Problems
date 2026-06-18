// Day 1684: Levenshtein edit distance via 1D rolling DP.
// Time O(n*m), Space O(min(n,m)).
#include <bits/stdc++.h>
using namespace std;

int editDistance(const string& a, const string& b) {
    int n = a.size(), m = b.size();
    vector<int> dp(m + 1);
    iota(dp.begin(), dp.end(), 0);
    for (int i = 1; i <= n; ++i) {
        int prev = dp[0];
        dp[0] = i;
        for (int j = 1; j <= m; ++j) {
            int tmp = dp[j];
            dp[j] = (a[i-1] == b[j-1]) ? prev
                  : 1 + min({prev, dp[j], dp[j-1]});
            prev = tmp;
        }
    }
    return dp[m];
}

int main() {
    cout << editDistance("kitten", "sitting") << "\n"; // 3
    return 0;
}
