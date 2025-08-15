// Day 124: Expected flipping rounds until one coin remains.
// DP: E[n](1-2^-n) = 1 + sum_{k<n} P(k survive)*E[k]. O(n^2) time, O(n) space.
#include <bits/stdc++.h>
using namespace std;

double expectedRounds(int n) {
    vector<double> E(n + 1, 0.0);
    // E[0] = E[1] = 0
    for (int m = 2; m <= n; m++) {
        // p_k = C(m,k)/2^m computed incrementally
        double p = pow(0.5, m); // p_0
        double s = 0.0;
        for (int k = 0; k < m; k++) {
            s += p * E[k];
            // p_{k+1} = p_k * (m-k)/(k+1)
            p = p * (double)(m - k) / (double)(k + 1);
        }
        double pn = pow(0.5, m); // P(all heads) = C(m,m)/2^m
        E[m] = (1.0 + s) / (1.0 - pn);
    }
    return E[n];
}

int main() {
    for (int n : {1, 2, 3, 4, 5, 10}) {
        printf("n=%-2d expected rounds = %.6f\n", n, expectedRounds(n));
    }
    return 0;
}
