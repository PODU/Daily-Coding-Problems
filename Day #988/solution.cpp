// Day 988: Minimum number of perfect squares summing to n.
// Number theory (Lagrange + Legendre's three-square theorem). O(sqrt n) time, O(1) space.
#include <bits/stdc++.h>
using namespace std;

bool isSquare(long long x) {
    long long r = (long long)sqrtl((long double)x);
    while (r * r > x) r--;
    while ((r + 1) * (r + 1) <= x) r++;
    return r * r == x;
}

int numSquares(long long n) {
    if (isSquare(n)) return 1;
    long long m = n;
    while (m % 4 == 0) m /= 4;          // strip factors of 4
    if (m % 8 == 7) return 4;           // Legendre: 4^a(8b+7) needs 4
    for (long long a = 1; a * a <= n; a++)
        if (isSquare(n - a * a)) return 2;
    return 3;
}

int main() {
    cout << "13 -> " << numSquares(13) << endl; // expected 2
    cout << "27 -> " << numSquares(27) << endl; // expected 3
    return 0;
}
