// LCS of three strings via 3D DP dp[i][j][k]. Time O(n1*n2*n3), Space O(n1*n2*n3).
#include <bits/stdc++.h>
using namespace std;

int lcs3(const string& a, const string& b, const string& c) {
    int n1 = a.size(), n2 = b.size(), n3 = c.size();
    vector<vector<vector<int>>> dp(n1 + 1, vector<vector<int>>(n2 + 1, vector<int>(n3 + 1, 0)));
    for (int i = 1; i <= n1; i++)
        for (int j = 1; j <= n2; j++)
            for (int k = 1; k <= n3; k++) {
                if (a[i - 1] == b[j - 1] && b[j - 1] == c[k - 1])
                    dp[i][j][k] = dp[i - 1][j - 1][k - 1] + 1;
                else
                    dp[i][j][k] = max({dp[i - 1][j][k], dp[i][j - 1][k], dp[i][j][k - 1]});
            }
    return dp[n1][n2][n3];
}

int main() {
    cout << lcs3("epidemiologist", "refrigeration", "supercalifragilisticexpialodocious") << "\n";
    return 0;
}
