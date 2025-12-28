// Monte Carlo pi: sample (x,y) in unit square via deterministic splitmix64 RNG,
// pi ~= 4*inside/total. Fixed seed -> deterministic. Time O(N), Space O(1).
#include <bits/stdc++.h>
using namespace std;

static uint64_t state;
uint64_t next_u64() {
    state += 0x9E3779B97F4A7C15ULL;
    uint64_t z = state;
    z = (z ^ (z >> 30)) * 0xBF58476D1CE4E5B9ULL;
    z = (z ^ (z >> 27)) * 0x94D049BB133111EBULL;
    return z ^ (z >> 31);
}
double next_double() { return (next_u64() >> 11) * (1.0 / 9007199254740992.0); }

int main() {
    state = 42ULL;
    const long long N = 10000000;
    long long inside = 0;
    for (long long i = 0; i < N; ++i) {
        double x = next_double(), y = next_double();
        if (x * x + y * y <= 1.0) inside++;
    }
    double pi = 4.0 * (double)inside / (double)N;
    printf("Estimated pi \xE2\x89\x88 %.3f\n", pi);
    return 0;
}
