// E[T] = sum_t (1 - (1-p)^n - n*p*(1-p)^(n-1)), p=2^-t (P(>1 coin alive after t rounds)). Sum until negligible. O(iterations) time, O(1) space.
#include <bits/stdc++.h>
using namespace std;

double expectedRounds(int n) {
    double total = 0.0;
    for (int t = 0; t < 1000; t++) {
        double p = pow(2.0, -t);
        double q = 1.0 - p;
        double term = 1.0 - pow(q, n) - n * p * pow(q, n - 1);
        total += term;
        if (t > 0 && term < 1e-15) break;
    }
    return total;
}

int main() {
    int n = 4;
    printf("Expected rounds for n=%d: %.4f\n", n, expectedRounds(n));
    return 0;
}
