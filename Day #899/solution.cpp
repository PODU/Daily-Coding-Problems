// 24 game, fixed order: recursively split the sequence at each position, combine left/right results with +,-,*,/ (eps for div). O(4^n) over splits; O(n) depth.
#include <bits/stdc++.h>
using namespace std;

const double EPS = 1e-6;

// Returns all reachable values for nums[lo..hi)
vector<double> solve(const vector<double>& nums, int lo, int hi) {
    if (hi - lo == 1) return {nums[lo]};
    vector<double> res;
    for (int mid = lo + 1; mid < hi; ++mid) {
        vector<double> L = solve(nums, lo, mid);
        vector<double> R = solve(nums, mid, hi);
        for (double a : L) for (double b : R) {
            res.push_back(a + b);
            res.push_back(a - b);
            res.push_back(a * b);
            if (fabs(b) > EPS) res.push_back(a / b);
        }
    }
    return res;
}

bool canReach24(const vector<int>& in) {
    vector<double> nums(in.begin(), in.end());
    for (double v : solve(nums, 0, (int)nums.size()))
        if (fabs(v - 24.0) < EPS) return true;
    return false;
}

int main() {
    vector<int> input = {5, 2, 7, 8};
    cout << (canReach24(input) ? "True" : "False") << "\n";
    return 0;
}
