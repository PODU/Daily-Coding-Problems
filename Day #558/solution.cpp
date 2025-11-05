// Estimate Pi via Monte Carlo: sample random points in unit square, fraction inside
// quarter circle ~ pi/4. O(S) for S samples. Fixed seed for reproducible 3-decimal result.
#include <bits/stdc++.h>
using namespace std;

double estimatePi(long long samples) {
    mt19937_64 rng(12345);
    uniform_real_distribution<double> dist(0.0, 1.0);
    long long inside = 0;
    for (long long i = 0; i < samples; ++i) {
        double x = dist(rng), y = dist(rng);
        if (x * x + y * y <= 1.0) ++inside;
    }
    return 4.0 * (double)inside / (double)samples;
}

int main() {
    printf("%.3f\n", estimatePi(20000000)); // ~3.142
    return 0;
}
