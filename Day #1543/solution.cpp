// Estimate pi via Monte Carlo: fraction of random points in [0,1]^2 inside the
// unit quarter-circle approximates pi/4. Time O(samples), Space O(1).
#include <bits/stdc++.h>
using namespace std;

double estimatePi(long long samples, unsigned seed) {
    mt19937_64 rng(seed);
    uniform_real_distribution<double> dist(0.0, 1.0);
    long long inside = 0;
    for (long long i = 0; i < samples; i++) {
        double x = dist(rng), y = dist(rng);
        if (x * x + y * y <= 1.0) inside++;
    }
    return 4.0 * inside / samples;
}

int main() {
    double pi = estimatePi(10000000LL, 42);
    cout << fixed << setprecision(3) << pi << "\n";
    return 0;
}
