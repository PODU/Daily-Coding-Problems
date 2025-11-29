// Day 670: Fewest perfect squares summing to n. Lagrange (answer in {1,2,3,4}) +
// Legendre three-square theorem. Time O(sqrt n), Space O(1).
#include <bits/stdc++.h>
using namespace std;

bool isSquare(long long n) { long long r = (long long)sqrtl((long double)n); return r * r == n || (r + 1) * (r + 1) == n; }

int numSquares(long long n) {
    if (isSquare(n)) return 1;
    for (long long a = 1; a * a <= n; a++) if (isSquare(n - a * a)) return 2;
    long long m = n;
    while (m % 4 == 0) m /= 4;
    if (m % 8 == 7) return 4;
    return 3;
}

int main() {
    cout << numSquares(13) << "\n"; // 2
    cout << numSquares(27) << "\n"; // 3
    return 0;
}
