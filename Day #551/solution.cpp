// Von Neumann fair-coin from biased coin: toss pairs, (0,1)->0, (1,0)->1, else retry.
// Expected O(1) tosses per fair toss; O(1) space.
#include <bits/stdc++.h>
using namespace std;

static uint64_t rng_state = 88172645463325252ULL;
static double next_uniform() { // xorshift64 -> [0,1)
    rng_state ^= rng_state << 13;
    rng_state ^= rng_state >> 7;
    rng_state ^= rng_state << 17;
    return (rng_state >> 11) * (1.0 / 9007199254740992.0);
}

int toss_biased() { // p(1) = 0.3
    return next_uniform() < 0.3 ? 1 : 0;
}

int fair_toss() {
    while (true) {
        int a = toss_biased();
        int b = toss_biased();
        if (a == 0 && b == 1) return 0;
        if (a == 1 && b == 0) return 1;
    }
}

int main() {
    long heads = 0, tails = 0;
    for (int i = 0; i < 100000; i++) {
        if (fair_toss() == 1) heads++; else tails++;
    }
    cout << "heads: " << heads << ", tails: " << tails << "\n";
    return 0;
}
