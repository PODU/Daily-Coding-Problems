// Min squared integers summing to n via Lagrange four-square + Legendre three-square theorem.
// O(sqrt(n)) per query (only the i^2+j^2 check loops), O(1) space.
#include <bits/stdc++.h>
using namespace std;

bool isPerfectSquare(long long n) {
    long long r = (long long)sqrtl((long double)n);
    while (r * r < n) r++;
    while (r * r > n) r--;
    return r * r == n;
}

int numSquares(long long n) {
    if (isPerfectSquare(n)) return 1;
    // Legendre: n = 4^a*(8b+7) => 4
    long long m = n;
    while (m % 4 == 0) m /= 4;
    if (m % 8 == 7) return 4;
    // check sum of two squares
    for (long long i = 1; i * i <= n; i++)
        if (isPerfectSquare(n - i * i)) return 2;
    return 3;
}

int main() {
    cout << numSquares(13) << "\n"; // 2
    cout << numSquares(27) << "\n"; // 3
    return 0;
}
