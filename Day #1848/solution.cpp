// Day 1848: Longest common subsequence of three strings via 3D dynamic programming.
// Time O(L1*L2*L3), Space O(L1*L2*L3).
#include <bits/stdc++.h>
using namespace std;

int lcs3(const string& a, const string& b, const string& c) {
    int la = a.size(), lb = b.size(), lc = c.size();
    vector<vector<vector<int>>> dp(la + 1,
        vector<vector<int>>(lb + 1, vector<int>(lc + 1, 0)));
    for (int i = 1; i <= la; i++)
        for (int j = 1; j <= lb; j++)
            for (int k = 1; k <= lc; k++) {
                if (a[i - 1] == b[j - 1] && b[j - 1] == c[k - 1])
                    dp[i][j][k] = dp[i - 1][j - 1][k - 1] + 1;
                else
                    dp[i][j][k] = max({dp[i - 1][j][k], dp[i][j - 1][k], dp[i][j][k - 1]});
            }
    return dp[la][lb][lc];
}

int main() {
    cout << lcs3("epidemiologist", "refrigeration",
                 "supercalifragilisticexpialodocious") << "\n";  // 5
}
