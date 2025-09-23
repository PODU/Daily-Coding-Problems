// Approximate median: median of k random samples (seeded RNG) -> rank in [N/4, 3N/4] whp.
// Time: O(k log k), o(N) for k<N; Space: O(k).
#include <bits/stdc++.h>
using namespace std;

int approxMedian(const vector<int>& a, int k, unsigned seed) {
    mt19937 rng(seed);
    uniform_int_distribution<int> dist(0, (int)a.size() - 1);
    vector<int> sample;
    for (int i = 0; i < k; i++) sample.push_back(a[dist(rng)]);
    sort(sample.begin(), sample.end());
    return sample[k / 2];
}

int main() {
    vector<int> a;
    for (int i = 0; i <= 100; i++) a.push_back(i); // N = 101, values 0..100
    int N = (int)a.size();
    int val = approxMedian(a, 15, 42);
    // rank of val in sorted 0..100 equals val itself
    int rank = val;
    bool ok = (N / 4 <= rank) && (rank <= (3 * N) / 4);
    cout << "Approximate median is within [N/4, 3N/4]: " << (ok ? "true" : "false") << "\n";
    return 0;
}
