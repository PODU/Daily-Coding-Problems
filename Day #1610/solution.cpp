// Reservoir sampling (reservoir size 1): for the i-th element replace kept with prob 1/i.
// Distribution is uniform over the stream. Time O(n), Space O(1). Seeded RNG -> deterministic.
#include <bits/stdc++.h>
using namespace std;

int reservoirSample(const vector<int>& stream, uint64_t seed) {
    mt19937_64 rng(seed);
    int kept = 0;
    for (int i = 0; i < (int)stream.size(); ++i) {
        // i is 0-indexed; for the (i+1)-th element keep with prob 1/(i+1)
        uniform_int_distribution<int> dist(0, i);
        if (dist(rng) == 0) kept = stream[i];
    }
    return kept;
}

int main() {
    vector<int> stream;
    for (int v = 1; v <= 10; ++v) stream.push_back(v);
    int selected = reservoirSample(stream, 42ULL);
    cout << "Selected: " << selected << "\n";
    return 0;
}
