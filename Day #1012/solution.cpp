// Square root via Newton's method: x = (x + n/x)/2, quadratic convergence.
// Time: O(log(1/eps)) iterations, Space: O(1).
#include <bits/stdc++.h>
using namespace std;

double mySqrt(double n) {
    if (n < 0) return nan("");
    if (n == 0) return 0;
    double x = n;            // initial guess
    for (int i = 0; i < 100; i++) {
        double nx = 0.5 * (x + n / x);
        if (fabs(nx - x) < 1e-15) { x = nx; break; }
        x = nx;
    }
    return x;
}

void printResult(double n) {
    double r = mySqrt(n);
    long long ri = llround(r);
    if (fabs(r - (double)ri) < 1e-9) cout << ri << "\n";          // exact integer
    else cout << fixed << setprecision(8) << r << "\n";
}

int main() {
    printResult(9);   // -> 3
    printResult(2);   // -> 1.41421356
    return 0;
}
