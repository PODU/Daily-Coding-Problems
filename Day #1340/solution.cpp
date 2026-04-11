// Count grid paths (right/down only) via combinatorics C(N+M-2, M-1).
// Overflow-safe multiplicative loop. Time O(N+M), Space O(1).
#include <iostream>
using namespace std;

unsigned long long countPaths(int n, int m) {
    int total = n + m - 2;
    int k = min(n - 1, m - 1);
    unsigned long long res = 1;
    for (int i = 1; i <= k; ++i) {
        res = res * (unsigned long long)(total - k + i) / (unsigned long long)i;
    }
    return res;
}

int main() {
    cout << "2 by 2 -> " << countPaths(2, 2) << "\n";
    cout << "5 by 5 -> " << countPaths(5, 5) << "\n";
    return 0;
}
