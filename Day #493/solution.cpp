// Day 493: Sample from a discrete distribution.
// Precompute cumulative probabilities; binary-search a uniform r in [0,1).
// Time: O(n) preprocessing, O(log n) per sample. Space: O(n).
#include <bits/stdc++.h>
using namespace std;

struct DiscreteSampler {
    vector<int> numbers;
    vector<double> cdf;
    DiscreteSampler(const vector<int>& nums, const vector<double>& probs)
        : numbers(nums), cdf(nums.size()) {
        double acc = 0;
        for (size_t i = 0; i < probs.size(); ++i) { acc += probs[i]; cdf[i] = acc; }
    }
    int sample(double r) const {
        // first index whose cdf > r
        int lo = 0, hi = (int)cdf.size() - 1;
        while (lo < hi) {
            int mid = (lo + hi) / 2;
            if (cdf[mid] > r) hi = mid; else lo = mid + 1;
        }
        return numbers[lo];
    }
};

int main() {
    vector<int> numbers = {1, 2, 3, 4};
    vector<double> probs = {0.1, 0.5, 0.2, 0.2};
    DiscreteSampler s(numbers, probs);

    mt19937 gen(42);
    uniform_real_distribution<double> dist(0.0, 1.0);
    const int N = 100000;
    map<int, int> counts;
    for (int i = 0; i < N; ++i) counts[s.sample(dist(gen))]++;

    cout << fixed << setprecision(3);
    for (int n : numbers)
        cout << n << ": " << (double)counts[n] / N << "\n";
    return 0;
}
