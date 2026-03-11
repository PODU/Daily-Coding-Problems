// Approximate median: take a constant-size random sample and return its median.
// Sublinear: O(k log k) for constant k samples, independent of N. Space: O(k).
#include <bits/stdc++.h>
using namespace std;

int approxMedian(const vector<int>& a, int& outRank) {
    int N = (int)a.size();
    int k = min(N, 31);
    mt19937 rng(42); // fixed seed for reproducibility
    vector<int> sample;
    sample.reserve(k);
    for (int i = 0; i < k; i++) {
        int idx = rng() % N;
        sample.push_back(a[idx]);
    }
    sort(sample.begin(), sample.end());
    int med = sample[sample.size() / 2];
    // rank = number of elements <= med (1-based rank of value)
    outRank = 0;
    for (int x : a) if (x <= med) outRank++;
    return med;
}

int main() {
    vector<int> a;
    for (int i = 1; i <= 100; i++) a.push_back(i);
    int rank;
    int v = approxMedian(a, rank);
    int N = (int)a.size();
    cout << "Approximate median: " << v << " (rank " << rank
         << " within [" << N / 4 << ", " << 3 * N / 4 << "])\n";
    return 0;
}
