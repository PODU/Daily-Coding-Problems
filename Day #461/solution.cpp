// Day 461: Number of right/down paths in an N x M grid = C(N+M-2, N-1).
// Multiplicative binomial. Time O(min(N,M)), Space O(1).
#include <iostream>
using namespace std;

long long countPaths(int n, int m) {
    int a = n - 1 + m - 1, b = min(n - 1, m - 1);
    long long res = 1;
    for (int i = 1; i <= b; i++) {
        res = res * (a - b + i) / i;
    }
    return res;
}

int main() {
    cout << countPaths(2, 2) << endl; // 2
    cout << countPaths(5, 5) << endl; // 70
    return 0;
}
