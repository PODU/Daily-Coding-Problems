// Count distinct max-heaps from N distinct values: ways(n)=C(n-1,L)*ways(L)*ways(R).
// L = size of left subtree of a complete binary tree with n nodes. Time: O(n^2). Space: O(n^2).
#include <bits/stdc++.h>
using namespace std;

int leftSubtreeSize(int n) {
    int h = (int)floor(log2((double)n));            // height (0-indexed) of full part
    int m = n - ((1 << h) - 1);                      // nodes in last level
    int lastCap = 1 << (h - 1);                       // capacity of left half of last level
    return ((1 << (h - 1)) - 1) + min(m, lastCap);
}

int main() {
    int N = 3;
    vector<long long> ways(N + 1, 0);
    vector<vector<long long>> C(N + 1, vector<long long>(N + 1, 0));
    for (int i = 0; i <= N; ++i) {
        C[i][0] = 1;
        for (int j = 1; j <= i; ++j) C[i][j] = C[i-1][j-1] + C[i-1][j];
    }
    ways[0] = ways[1] = 1;
    for (int n = 2; n <= N; ++n) {
        int L = leftSubtreeSize(n);
        int R = n - 1 - L;
        ways[n] = C[n-1][L] * ways[L] * ways[R];
    }
    cout << ways[N] << endl;
    return 0;
}
