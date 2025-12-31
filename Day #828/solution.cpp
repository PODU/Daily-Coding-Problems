// Day 828: Count distinct max heaps from N distinct integers.
// Root is the max; f(n) = C(n-1, L) * f(L) * f(R), L = left-subtree size from
// complete-tree shape. Time O(N^2) (Pascal), Space O(N^2).
#include <bits/stdc++.h>
using namespace std;

vector<vector<long long>> C;

int leftCount(int m) {
    if (m == 1) return 0;
    int h = 31 - __builtin_clz(m);        // height (0-indexed levels)
    int last = m - ((1 << h) - 1);        // nodes in last level
    int leftCap = 1 << (h - 1);
    return ((1 << (h - 1)) - 1) + min(leftCap, last);
}

long long f(int m) {
    if (m <= 1) return 1;
    int L = leftCount(m);
    int R = m - 1 - L;
    return C[m - 1][L] * f(L) * f(R);
}

long long countMaxHeaps(int n) {
    C.assign(n + 1, vector<long long>(n + 1, 0));
    for (int i = 0; i <= n; i++) {
        C[i][0] = 1;
        for (int j = 1; j <= i; j++) C[i][j] = C[i - 1][j - 1] + C[i - 1][j];
    }
    return f(n);
}

int main() {
    cout << countMaxHeaps(3) << "\n";
    return 0;
}
