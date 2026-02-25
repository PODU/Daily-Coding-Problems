// Smallest number of perfect squares summing to N.
// Legendre/Lagrange: 1 if perfect square; 4 if N=4^a(8b+7); 2 if sum of two squares; else 3. O(sqrt(N)) time, O(1) space.
#include <bits/stdc++.h>
using namespace std;

bool isSquare(long long n) { long long r = (long long)sqrt((double)n); while (r*r < n) r++; while (r*r > n) r--; return r*r == n; }

int numSquares(long long n) {
    if (isSquare(n)) return 1;
    long long m = n;
    while (m % 4 == 0) m /= 4;
    if (m % 8 == 7) return 4;
    for (long long a = 1; a*a <= n; a++) if (isSquare(n - a*a)) return 2;
    return 3;
}

int main() {
    long long N = 17;
    cout << numSquares(N) << endl;
    return 0;
}
