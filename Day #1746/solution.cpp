// Day 1746: Weighted random sampler.
// Approach: prefix-sum of probabilities + binary search on a uniform U[0,1).
// Build O(n), sample O(log n) time, O(n) space.
#include <bits/stdc++.h>
using namespace std;

struct WeightedSampler {
    vector<int> nums;
    vector<double> cdf;
    mt19937 rng;
    uniform_real_distribution<double> uni{0.0, 1.0};

    WeightedSampler(vector<int> n, vector<double> p, unsigned seed = 42)
        : nums(move(n)), rng(seed) {
        cdf.resize(p.size());
        double acc = 0;
        for (size_t i = 0; i < p.size(); ++i) { acc += p[i]; cdf[i] = acc; }
    }
    int sample() {
        double r = uni(rng);
        int idx = lower_bound(cdf.begin(), cdf.end(), r) - cdf.begin();
        if (idx >= (int)nums.size()) idx = nums.size() - 1;
        return nums[idx];
    }
};

int main() {
    vector<int> numbers = {1, 2, 3, 4};
    vector<double> probs = {0.1, 0.5, 0.2, 0.2};
    WeightedSampler s(numbers, probs);

    const int N = 1000000;
    map<int,int> cnt;
    for (int i = 0; i < N; ++i) cnt[s.sample()]++;

    cout << "Observed frequencies over " << N << " samples:\n";
    for (int x : numbers)
        cout << x << ": " << fixed << setprecision(1)
             << (100.0 * cnt[x] / N) << "%\n";
    cout << "Expected: 1 10% of the time, 2 50% of the time, and 3 and 4 20% of the time\n";
    return 0;
}
