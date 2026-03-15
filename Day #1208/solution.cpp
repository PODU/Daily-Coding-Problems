// Day 1208: Fewest perfect squares summing to N.
// Lagrange four-square + Legendre's three-square theorem. Time O(sqrt N), Space O(1).
#include <bits/stdc++.h>
using namespace std;

bool isSquare(long long n) { long long r = (long long)sqrtl((long double)n); while (r * r < n) r++; while (r * r > n) r--; return r * r == n; }

int numSquares(long long n) {
    if (isSquare(n)) return 1;
    for (long long a = 1; a * a <= n; a++) if (isSquare(n - a * a)) return 2; // sum of two squares
    long long m = n;
    while (m % 4 == 0) m /= 4;            // remove factors of 4
    if (m % 8 == 7) return 4;             // Legendre: 4^a(8b+7) needs 4
    return 3;
}

int main() {
    cout << numSquares(17) << "\n"; // 2  (16 + 1)
    return 0;
}
