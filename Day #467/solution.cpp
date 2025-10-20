// Square root via Newton's method: x = (x + n/x)/2 until convergence.
// Time: O(log(1/eps)) iterations, Space: O(1).
#include <bits/stdc++.h>
using namespace std;

double mySqrt(double n) {
    if (n < 0) return NAN;
    if (n == 0) return 0;
    double x = n;
    for (int i = 0; i < 100; i++) {
        double nx = 0.5 * (x + n / x);
        if (fabs(nx - x) < 1e-12) break;
        x = nx;
    }
    return x;
}

int main() {
    double n = 9;
    double r = mySqrt(n);
    if (fabs(r - round(r)) < 1e-9)
        cout << (long long)round(r) << endl;
    else
        cout << r << endl;
    return 0;
}
