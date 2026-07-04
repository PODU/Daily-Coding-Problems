// Day 1762: Count parenthesizations of a boolean expression evaluating to True.
// Interval DP over operands: t[i][j]/f[i][j] = #ways range evals True/False,
// combine across each split operator. Time O(n^3), Space O(n^2).
#include <bits/stdc++.h>
using namespace std;

long long countTrue(const vector<string>& tokens) {
    vector<char> vals, ops;
    for (size_t i = 0; i < tokens.size(); ++i)
        (i % 2 == 0 ? vals : ops).push_back(tokens[i][0]);
    int n = vals.size();
    vector<vector<long long>> t(n, vector<long long>(n, 0)), f(n, vector<long long>(n, 0));
    for (int i = 0; i < n; ++i) {
        t[i][i] = (vals[i] == 'T');
        f[i][i] = (vals[i] == 'F');
    }
    for (int len = 2; len <= n; ++len)
        for (int i = 0; i + len - 1 < n; ++i) {
            int j = i + len - 1;
            for (int k = i; k < j; ++k) {
                char op = ops[k];
                long long lt = t[i][k], lf = f[i][k], rt = t[k + 1][j], rf = f[k + 1][j];
                long long tot = (lt + lf) * (rt + rf);
                if (op == '&') { t[i][j] += lt * rt; f[i][j] += tot - lt * rt; }
                else if (op == '|') { f[i][j] += lf * rf; t[i][j] += tot - lf * rf; }
                else { t[i][j] += lt * rf + lf * rt; f[i][j] += lt * rt + lf * rf; }
            }
        }
    return t[0][n - 1];
}

int main() {
    vector<string> tokens = {"F", "|", "T", "&", "T"};
    cout << countTrue(tokens) << "\n";
    return 0;
}
