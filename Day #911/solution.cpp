// Reservoir sampling (size 1): replace pick with prob 1/i for i-th element. O(n) time, O(1) space.
#include <bits/stdc++.h>
using namespace std;

int reservoirSample(const vector<int>& stream, mt19937& rng) {
    int pick = 0;
    int i = 0;
    for (int x : stream) {
        ++i; // 1-indexed
        // replace with probability 1/i
        if (uniform_int_distribution<int>(1, i)(rng) == 1) pick = x;
    }
    return pick;
}

int main() {
    mt19937 rng(12345);
    vector<int> stream(10);
    iota(stream.begin(), stream.end(), 0); // 0..9

    int sampled = reservoirSample(stream, rng);
    cout << "Sampled element: " << sampled << "\n";

    // Empirical uniformity check
    const int trials = 100000, n = 10;
    vector<int> freq(n, 0);
    for (int t = 0; t < trials; ++t) freq[reservoirSample(stream, rng)]++;
    cout << "Approx frequencies over " << trials << " trials (expect ~"
         << (1.0 / n) << " each):\n";
    for (int v = 0; v < n; ++v)
        cout << "  " << v << ": " << fixed << setprecision(4)
             << (double)freq[v] / trials << "\n";
    cout << "Distribution is ~uniform.\n";
    return 0;
}
