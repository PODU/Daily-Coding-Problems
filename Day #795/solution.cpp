// Von Neumann debiasing: toss biased coin twice; (0,1)->0, (1,0)->1, else retry.
// Output is provably fair regardless of bias. O(1) expected tosses per fair bit.
#include <iostream>
#include <iomanip>
using namespace std;

static unsigned long long state = 123456789ULL;
static double next_rand() { // LCG in [0,1)
    state = state * 6364136223846793005ULL + 1442695040888963407ULL;
    return (state >> 11) / (double)(1ULL << 53);
}

int toss_biased() { return next_rand() < 0.3 ? 1 : 0; } // P(1) = 0.3

int fair_coin() {
    while (true) {
        int a = toss_biased();
        int b = toss_biased();
        if (a == 0 && b == 1) return 0;
        if (a == 1 && b == 0) return 1;
    }
}

int main() {
    const int N = 100000;
    int ones = 0;
    for (int i = 0; i < N; ++i) ones += fair_coin();
    double p = (double)ones / N;
    cout << fixed << setprecision(1) << p << '\n';
    return 0;
}
