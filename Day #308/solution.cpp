// Day 308: Count parenthesizations evaluating True. Interval DP. O(n^3) time, O(n^2) space.
#include <bits/stdc++.h>
using namespace std;
long long countTrue(vector<string> e) {
    int n = e.size(), V = (n + 1) / 2; // operands at even indices
    vector<vector<long long>> T(V, vector<long long>(V, 0)), F(V, vector<long long>(V, 0));
    for (int i = 0; i < V; i++) { bool val = e[2 * i] == "T"; T[i][i] = val; F[i][i] = !val; }
    for (int len = 2; len <= V; len++) for (int i = 0; i + len - 1 < V; i++) {
        int j = i + len - 1;
        for (int k = i; k < j; k++) {
            string op = e[2 * k + 1];
            long long lt = T[i][k], lf = F[i][k], rt = T[k + 1][j], rf = F[k + 1][j];
            long long total = (lt + lf) * (rt + rf), t = 0;
            if (op == "&") t = lt * rt;
            else if (op == "|") t = lt * rt + lt * rf + lf * rt;
            else t = lt * rf + lf * rt; // ^
            T[i][j] += t; F[i][j] += total - t;
        }
    }
    return T[0][V - 1];
}
int main() {
    vector<string> e = {"F", "|", "T", "&", "T"};
    cout << countTrue(e) << "\n"; // 2
}
