// Square root of a real number via Newton's method: x <- (x + n/x)/2.
// Quadratic convergence -> O(log(1/eps)) iterations, O(1) space.
#include <bits/stdc++.h>
using namespace std;

double mySqrt(double n) {
    if (n < 0) throw runtime_error("negative");
    if (n == 0) return 0;
    double x = n; // initial guess
    for (int i = 0; i < 100; i++) {
        double nx = 0.5 * (x + n / x);
        if (fabs(nx - x) < 1e-15) break;
        x = nx;
    }
    return x;
}

int main() {
    double r = mySqrt(9);
    if (fabs(r - llround(r)) < 1e-9) cout << llround(r) << "\n"; // 3
    else cout << setprecision(12) << r << "\n";
    return 0;
}
