// Interval DP: T[i][j]/F[i][j] = #ways subexpr of operands i..j is True/False.
// Split at each operator, combine child counts per &,|,^. Time O(n^3), Space O(n^2).
#include <bits/stdc++.h>
using namespace std;

int countTrue(const vector<string>& expr) {
    vector<char> vals; vector<char> ops;
    for (size_t i = 0; i < expr.size(); i++)
        if (i % 2 == 0) vals.push_back(expr[i][0]);
        else ops.push_back(expr[i][0]);
    int n = vals.size();
    if (n == 0) return 0;
    vector<vector<long long>> T(n, vector<long long>(n, 0)), F(n, vector<long long>(n, 0));
    for (int i = 0; i < n; i++) { T[i][i] = vals[i] == 'T'; F[i][i] = vals[i] == 'F'; }
    for (int len = 2; len <= n; len++)
        for (int i = 0; i + len - 1 < n; i++) {
            int j = i + len - 1;
            for (int k = i; k < j; k++) {
                char op = ops[k];
                long long lt = T[i][k], lf = F[i][k], rt = T[k+1][j], rf = F[k+1][j];
                long long tot = (lt + lf) * (rt + rf);
                long long t = 0;
                if (op == '&') t = lt * rt;
                else if (op == '|') t = lt * rt + lt * rf + lf * rt;
                else t = lt * rf + lf * rt; // ^
                T[i][j] += t;
                F[i][j] += tot - t;
            }
        }
    return (int)T[0][n - 1];
}

int main() {
    vector<string> expr = {"F", "|", "T", "&", "T"};
    cout << countTrue(expr) << "\n"; // 2
    return 0;
}
