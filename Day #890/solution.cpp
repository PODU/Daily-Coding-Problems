// Lattice paths in N x M grid = C(N+M-2, N-1), computed iteratively.
// Time: O(min(N,M)), Space: O(1).
#include <bits/stdc++.h>
using namespace std;

long long paths(long long n, long long m) {
    long long total = n + m - 2;
    long long k = min(n - 1, m - 1);
    long long res = 1;
    for (long long i = 1; i <= k; i++) {
        res = res * (total - k + i) / i;
    }
    return res;
}

int main() {
    cout << paths(2, 2) << "\n";
    cout << paths(5, 5) << "\n";
    return 0;
}
