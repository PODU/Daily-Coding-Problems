// Von Neumann extractor: toss biased coin twice; (0,1)->0, (1,0)->1, else retry => 50/50. Time O(1) expected.
#include <bits/stdc++.h>
using namespace std;

static std::mt19937 rng(12345);

int toss_biased() { // simulated bias p = 0.3 for 1
    std::uniform_real_distribution<double> d(0.0, 1.0);
    return d(rng) < 0.3 ? 1 : 0;
}

int toss_fair() {
    while (true) {
        int a = toss_biased();
        int b = toss_biased();
        if (a == 0 && b == 1) return 0;
        if (a == 1 && b == 0) return 1;
    }
}

int main() {
    const int trials = 100000;
    long ones = 0;
    for (int i = 0; i < trials; ++i) ones += toss_fair();
    double frac = (double)ones / trials;
    assert(frac > 0.48 && frac < 0.52);
    cout << "Fair coin ~0.5" << "\n";
    return 0;
}
