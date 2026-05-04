// Least squares summing to n via Legendre/Lagrange: 1 if square, 4 if 4^a(8b+7),
// 2 if i^2+j^2, else 3. Time: O(sqrt(n)); Space: O(1).
#include <bits/stdc++.h>
using namespace std;

bool isSquare(long long x) {
    long long r = (long long)sqrtl((long double)x);
    while (r * r < x) r++;
    while (r * r > x) r--;
    return r * r == x;
}

int numSquares(long long n) {
    if (isSquare(n)) return 1;
    long long m = n;
    while (m % 4 == 0) m /= 4;
    if (m % 8 == 7) return 4;
    for (long long i = 1; i * i <= n; i++)
        if (isSquare(n - i * i)) return 2;
    return 3;
}

int main() {
    cout << numSquares(13) << "\n";
    cout << numSquares(27) << "\n";
    return 0;
}
