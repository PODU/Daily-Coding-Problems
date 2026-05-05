// Day 1473: Count occurrences of X in an N x N multiplication table.
// For each row i, X appears iff i divides X and X/i is within [1, N].
// Time O(N), Space O(1).
#include <bits/stdc++.h>
using namespace std;

long long countX(long long n, long long x) {
    long long count = 0;
    for (long long i = 1; i <= n; ++i) {
        if (x % i == 0 && x / i >= 1 && x / i <= n) ++count;
    }
    return count;
}

int main() {
    cout << countX(6, 12) << "\n";  // 4
}
