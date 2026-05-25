// Paths in grid = C(N+M-2, N-1), computed multiplicatively to avoid overflow. Time O(min(N,M)), Space O(1).
#include <bits/stdc++.h>
using namespace std;

long long countPaths(int n, int m) {
    int total = n + m - 2;
    int k = min(n - 1, m - 1);
    long long res = 1;
    for (int i = 1; i <= k; ++i) {
        res = res * (total - k + i) / i;
    }
    return res;
}

int main() {
    cout << countPaths(2, 2) << endl;
    return 0;
}
