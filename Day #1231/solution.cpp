// Monte Carlo pi estimate with a shared 64-bit LCG. Time O(n), Space O(1).
#include <cstdint>
#include <cstdio>
using namespace std;

static const uint64_t A = 6364136223846793005ULL;
static const uint64_t C = 1442695040888963407ULL;

double estimate_pi(long long samples, uint64_t seed = 42) {
    uint64_t x = seed;
    long long inside = 0;
    for (long long i = 0; i < samples; ++i) {
        x = A * x + C;
        double px = (double)(x >> 11) / (double)(1ULL << 53);
        x = A * x + C;
        double py = (double)(x >> 11) / (double)(1ULL << 53);
        if (px * px + py * py <= 1.0) ++inside;
    }
    return 4.0 * inside / samples;
}

int main() {
    printf("%.3f\n", estimate_pi(2000000));
    return 0;
}
