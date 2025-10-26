// Day 495: Reservoir sampling (size 1) from a stream of unknown length.
// For the i-th element (1-indexed) keep it with probability 1/i. Uses O(1) memory.
// Time: O(n) per pass, Space: O(1).
#include <bits/stdc++.h>
using namespace std;

// Processes a stream via an iterator without storing it; rng() returns a uniform double in [0,1).
template <class It, class Rng>
int reservoirSample(It begin, It end, Rng& rng) {
    int chosen = 0, count = 0;
    for (It it = begin; it != end; ++it) {
        ++count;
        if (rng() < 1.0 / count) chosen = *it;
    }
    return chosen;
}

int main() {
    vector<int> stream;
    for (int i = 1; i <= 10; ++i) stream.push_back(i);

    mt19937 gen(42);
    uniform_real_distribution<double> dist(0.0, 1.0);
    auto rng = [&]() { return dist(gen); };

    const int TRIALS = 100000;
    map<int, int> counts;
    for (int t = 0; t < TRIALS; ++t)
        counts[reservoirSample(stream.begin(), stream.end(), rng)]++;

    cout << fixed << setprecision(3);
    cout << "Empirical selection frequency per element (~0.100 each):\n";
    for (int v : stream)
        cout << v << ": " << (double)counts[v] / TRIALS << "\n";
    cout << "One sampled value: "
         << reservoirSample(stream.begin(), stream.end(), rng) << "\n";
    return 0;
}
