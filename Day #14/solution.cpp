// Estimate pi via Monte Carlo: fraction of random points in unit circle * 4.
// Time: O(samples), Space: O(1). Seeded for reproducible 3-decimal output.
#include <bits/stdc++.h>
using namespace std;

double estimatePi(long long samples) {
    mt19937_64 rng(12345);
    uniform_real_distribution<double> dist(0.0, 1.0);
    long long inside = 0;
    for (long long i = 0; i < samples; i++) {
        double x = dist(rng), y = dist(rng);
        if (x * x + y * y <= 1.0) inside++;
    }
    return 4.0 * inside / samples;
}

int main() {
    printf("%.3f\n", estimatePi(10000000));
    return 0;
}
