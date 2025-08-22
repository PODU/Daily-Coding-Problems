// Day 152: Weighted random sampling. Build cumulative distribution, draw u in
// [0,1) and binary-search the bucket. Preprocess O(n), per-sample O(log n).
#include <bits/stdc++.h>
using namespace std;

struct WeightedSampler {
    vector<int> nums;
    vector<double> cdf;
    mt19937 rng;
    uniform_real_distribution<double> dist{0.0, 1.0};

    WeightedSampler(vector<int> n, vector<double> p, unsigned seed)
        : nums(move(n)), rng(seed) {
        double acc = 0;
        for (double x : p) { acc += x; cdf.push_back(acc); }
    }
    int sample() {
        double u = dist(rng);
        int idx = lower_bound(cdf.begin(), cdf.end(), u) - cdf.begin();
        if (idx >= (int)nums.size()) idx = nums.size() - 1;
        return nums[idx];
    }
};

int main() {
    vector<int> nums = {1, 2, 3, 4};
    vector<double> probs = {0.1, 0.5, 0.2, 0.2};
    WeightedSampler s(nums, probs, 42);
    const int N = 1000000;
    map<int,int> cnt;
    for (int i = 0; i < N; i++) cnt[s.sample()]++;
    cout << fixed << setprecision(2);
    for (int n : nums)
        cout << n << ": " << (100.0 * cnt[n] / N) << "%\n";
    return 0;
}
