// Day 1728: Simulate a fair coin from a biased one (Von Neumann trick).
// Toss biased coin twice; (0,1)->0, (1,0)->1, else retry. Expected P(heads) ~= 0.5.
// Time: O(1) expected tosses per fair() call. Space: O(1).
#include <bits/stdc++.h>
using namespace std;

static mt19937 rng(12345);

// Biased coin: returns 1 with probability p (= 0.3), else 0.
int toss_biased() {
    static uniform_real_distribution<double> dist(0.0, 1.0);
    return dist(rng) < 0.3 ? 1 : 0;
}

// Von Neumann: extract a fair bit from the biased coin.
int fair() {
    while (true) {
        int a = toss_biased();
        int b = toss_biased();
        if (a == 0 && b == 1) return 0;
        if (a == 1 && b == 0) return 1;
        // (0,0) or (1,1): discard and retry.
    }
}

int main() {
    const int N = 100000;
    long long heads = 0;
    for (int i = 0; i < N; i++) heads += fair();
    double ratio = (double)heads / N;
    cout << "Fair coin over " << N << " tosses, P(heads) ~= "
         << fixed << setprecision(2) << ratio << "\n";
    return 0;
}
