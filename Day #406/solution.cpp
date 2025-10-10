// Boolean parenthesization: count ways the expression evaluates to True.
// Interval DP storing #True/#False per substring. Time O(n^3), Space O(n^2).
#include <bits/stdc++.h>
using namespace std;

long long countTrue(const vector<string>& a) {
    vector<char> val, op;
    for (size_t i = 0; i < a.size(); i++) {
        if (i % 2 == 0) val.push_back(a[i][0]);
        else op.push_back(a[i][0]);
    }
    int M = val.size();
    vector<vector<long long>> T(M, vector<long long>(M, 0)), F(M, vector<long long>(M, 0));
    for (int i = 0; i < M; i++) { T[i][i] = (val[i] == 'T'); F[i][i] = (val[i] == 'F'); }
    for (int len = 2; len <= M; len++)
        for (int i = 0; i + len - 1 < M; i++) {
            int j = i + len - 1;
            for (int k = i; k < j; k++) {
                char o = op[k];
                long long lt = T[i][k], lf = F[i][k], rt = T[k + 1][j], rf = F[k + 1][j];
                long long tot = (lt + lf) * (rt + rf);
                if (o == '&') { T[i][j] += lt * rt; F[i][j] += tot - lt * rt; }
                else if (o == '|') { T[i][j] += tot - lf * rf; F[i][j] += lf * rf; }
                else { T[i][j] += lt * rf + lf * rt; F[i][j] += lt * rt + lf * rf; }
            }
        }
    return T[0][M - 1];
}

int main() {
    vector<string> expr = {"F", "|", "T", "&", "T"};
    cout << countTrue(expr) << endl;
    return 0;
}
