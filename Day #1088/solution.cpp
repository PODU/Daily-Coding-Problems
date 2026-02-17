// Boolean parenthesization via interval DP counting True/False groupings. Time O(n^3), Space O(n^2).
#include <bits/stdc++.h>
using namespace std;

long long countTrue(const vector<char>& expr) {
    int m = expr.size();
    int n = (m + 1) / 2;
    vector<bool> val(n);
    vector<char> ops(n > 0 ? n - 1 : 0);
    for (int i = 0; i < m; i++) {
        if (i % 2 == 0) val[i / 2] = (expr[i] == 'T');
        else ops[i / 2] = expr[i];
    }
    vector<vector<long long>> T(n, vector<long long>(n, 0)), F(n, vector<long long>(n, 0));
    for (int i = 0; i < n; i++) { T[i][i] = val[i] ? 1 : 0; F[i][i] = val[i] ? 0 : 1; }
    for (int len = 2; len <= n; len++)
        for (int i = 0; i + len - 1 < n; i++) {
            int j = i + len - 1;
            for (int k = i; k < j; k++) {
                char op = ops[k];
                long long lt = T[i][k], lf = F[i][k], rt = T[k + 1][j], rf = F[k + 1][j];
                if (op == '&') { T[i][j] += lt * rt; F[i][j] += lf * rf + lf * rt + lt * rf; }
                else if (op == '|') { T[i][j] += lt * rt + lt * rf + lf * rt; F[i][j] += lf * rf; }
                else { T[i][j] += lt * rf + lf * rt; F[i][j] += lt * rt + lf * rf; }
            }
        }
    return T[0][n - 1];
}

int main() {
    vector<char> expr = {'F', '|', 'T', '&', 'T'};
    cout << countTrue(expr) << "\n";
}
