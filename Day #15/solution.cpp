// Reservoir sampling (k=1): keep current pick with prob 1/i as i-th item streams.
// Time: O(n) single pass, Space: O(1).
#include <bits/stdc++.h>
using namespace std;

template <class It>
int pickRandom(It begin, It end, mt19937& rng) {
    int chosen = 0, count = 0;
    for (It it = begin; it != end; ++it) {
        count++;
        if (uniform_int_distribution<int>(0, count - 1)(rng) == 0) chosen = *it;
    }
    return chosen;
}

int main() {
    vector<int> stream = {10, 20, 30, 40, 50};
    mt19937 rng(42);
    // Demonstrate uniformity over many trials (each element ~20%).
    map<int, int> freq;
    for (int t = 0; t < 100000; t++) freq[pickRandom(stream.begin(), stream.end(), rng)]++;
    cout << "One sample: " << pickRandom(stream.begin(), stream.end(), rng) << "\n";
    cout << "Approx frequencies over 100000 trials:\n";
    for (auto& kv : freq) printf("  %d: %.3f\n", kv.first, kv.second / 100000.0);
    return 0;
}
