// Day 527: Count distinct max-heaps from N distinct integers.
// Recurrence f(n) = C(n-1, L) * f(L) * f(R), L = left-subtree size of a complete
// binary tree with n nodes. Time O(n^2) for Pascal's triangle, space O(n^2).
#include <bits/stdc++.h>
using namespace std;

// number of nodes in the left subtree of a complete binary tree with n nodes
long long leftSubtreeSize(long long n) {
    if (n <= 1) return 0;
    int h = 0;
    while ((1LL << (h + 1)) - 1 <= n) h++; // h = height (root at height 0)
    long long lastLevelCap = 1LL << h;             // capacity of the last level
    long long nodesAbove = (1LL << h) - 1;          // nodes in the upper h levels
    long long lastLevelNodes = n - nodesAbove;      // actual nodes on the last level
    long long leftBase = (1LL << (h - 1)) - 1;      // full left subtree above last level
    long long leftLast = min(lastLevelNodes, lastLevelCap / 2);
    return leftBase + leftLast;
}

long long C[64][64];
void buildPascal(int n) {
    for (int i = 0; i <= n; i++) {
        C[i][0] = 1;
        for (int j = 1; j <= i; j++)
            C[i][j] = C[i - 1][j - 1] + (j <= i - 1 ? C[i - 1][j] : 0);
    }
}

long long countHeaps(long long n) {
    if (n <= 1) return 1;
    long long L = leftSubtreeSize(n);
    long long R = n - 1 - L;
    return C[n - 1][L] * countHeaps(L) * countHeaps(R);
}

int main() {
    int N = 3;
    vector<int> integers = {1, 2, 3};
    buildPascal(N + 1);
    cout << countHeaps(N) << "\n"; // expected: 2
    return 0;
}
