// Count distinct max-heaps from N distinct integers. ways(n)=C(n-1,L)*ways(L)*ways(R),
// L = left-subtree node count of a complete binary tree of n nodes. Time O(n^2), Space O(n^2).
#include <bits/stdc++.h>
using namespace std;

long long C[64][64];

int leftCount(int n) {
    if (n == 1) return 0;
    int h = (int)floor(log2((double)n));      // height (root at level 0)
    int last = n - ((1 << h) - 1);            // nodes in the bottom level
    int maxLast = 1 << (h - 1);               // max bottom-level nodes for left subtree
    int left = ((1 << (h - 1)) - 1) + min(last, maxLast);
    return left;
}

long long ways(int n) {
    if (n <= 1) return 1;
    int L = leftCount(n), R = n - 1 - L;
    return C[n - 1][L] * ways(L) * ways(R);
}

int main() {
    for (int i = 0; i < 64; i++) {
        C[i][0] = 1;
        for (int j = 1; j <= i; j++) C[i][j] = C[i - 1][j - 1] + C[i - 1][j];
    }
    int arr[] = {1, 2, 3};               // N distinct integers
    int n = sizeof(arr) / sizeof(arr[0]);
    cout << ways(n) << "\n";
    return 0;
}
