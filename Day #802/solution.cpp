// Day 802: Sample a number per given probability distribution.
// Build CDF prefix sums once O(n); each sample draws u~U(0,1) + binary search O(log n).
#include <bits/stdc++.h>
using namespace std;

struct WeightedSampler {
    vector<int> nums;
    vector<double> cdf;
    mt19937 rng;
    uniform_real_distribution<double> uni{0.0, 1.0};

    WeightedSampler(vector<int> n, vector<double> p, unsigned seed)
        : nums(move(n)), rng(seed) {
        double acc = 0;
        for (double x : p) { acc += x; cdf.push_back(acc); }
    }
    int sample() {
        double u = uni(rng);
        int i = lower_bound(cdf.begin(), cdf.end(), u) - cdf.begin();
        return nums[min(i, (int)nums.size() - 1)];
    }
};

int main() {
    vector<int> numbers = {1, 2, 3, 4};
    vector<double> probs = {0.1, 0.5, 0.2, 0.2};
    WeightedSampler s(numbers, probs, 42);
    map<int, int> count;
    int trials = 100000;
    for (int i = 0; i < trials; i++) count[s.sample()]++;
    cout << fixed << setprecision(2);
    for (int n : numbers) cout << n << ": " << (double)count[n] / trials << "\n";
    // ~ 1:0.10  2:0.50  3:0.20  4:0.20
    return 0;
}
