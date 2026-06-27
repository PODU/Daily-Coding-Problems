// Day 1722: Approximate median via random sampling.
// Sample a sublinear number of elements (~constant), return their exact median.
// With high probability its rank lies in [N/4, 3N/4]. Time: O(s log s) << O(N), Space: O(s).
#include <bits/stdc++.h>
using namespace std;

int approxMedian(const vector<int>& a, mt19937& rng) {
    int n = (int)a.size();
    int s = min(n, 99); // sublinear sample size
    vector<int> sample;
    sample.reserve(s);
    uniform_int_distribution<int> dist(0, n - 1);
    for (int i = 0; i < s; ++i) sample.push_back(a[dist(rng)]);
    sort(sample.begin(), sample.end());
    return sample[s / 2];
}

int main() {
    // Demo: values 0..99 shuffled deterministically.
    int N = 100;
    vector<int> a(N);
    iota(a.begin(), a.end(), 0);
    mt19937 rng(42);
    shuffle(a.begin(), a.end(), rng);

    int m = approxMedian(a, rng);
    // rank = number of elements < m (values are 0..99, so rank == value here).
    int rank = (int)count_if(a.begin(), a.end(), [&](int x){ return x < m; });
    cout << "Approximate median: " << m
         << " (rank " << rank << " within [N/4, 3N/4] = ["
         << N / 4 << ", " << 3 * N / 4 << "])\n";
    return 0;
}
