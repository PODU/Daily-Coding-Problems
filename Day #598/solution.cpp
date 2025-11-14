// Day 598: Expected number of rounds flipping n coins (remove tails) until one remains.
// DP: f(n) = (1 + 2^-n * sum_{k<n} C(n,k) f(k)) / (1 - 2^-n). Time O(n^2), Space O(n^2).
#include <bits/stdc++.h>
using namespace std;

double expectedRounds(int n) {
    if (n <= 1) return 0.0;
    // binomial coefficients via Pascal's triangle
    vector<vector<double>> C(n + 1, vector<double>(n + 1, 0.0));
    for (int i = 0; i <= n; i++) {
        C[i][0] = 1.0;
        for (int j = 1; j <= i; j++)
            C[i][j] = C[i - 1][j - 1] + C[i - 1][j];
    }
    vector<double> f(n + 1, 0.0);
    for (int m = 2; m <= n; m++) {
        double half = pow(0.5, m);
        double s = 0.0;
        for (int k = 0; k < m; k++) s += C[m][k] * f[k];
        f[m] = (1.0 + half * s) / (1.0 - half);
    }
    return f[n];
}

int main() {
    int n = 4;
    cout << fixed << setprecision(4) << expectedRounds(n) << "\n"; // ~2.0571
    return 0;
}
