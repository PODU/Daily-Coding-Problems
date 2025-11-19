// Split array into k contiguous parts minimizing the max partition sum.
// Binary search on answer in [max, total]; greedy feasibility check. O(n log(sum)).
#include <bits/stdc++.h>
using namespace std;

bool feasible(const vector<int>& nums, int k, long long cap) {
    int parts = 1;
    long long cur = 0;
    for (int x : nums) {
        if (cur + x > cap) { parts++; cur = x; }
        else cur += x;
    }
    return parts <= k;
}

long long splitArray(const vector<int>& nums, int k) {
    long long lo = *max_element(nums.begin(), nums.end());
    long long hi = accumulate(nums.begin(), nums.end(), 0LL);
    while (lo < hi) {
        long long mid = lo + (hi - lo) / 2;
        if (feasible(nums, k, mid)) hi = mid;
        else lo = mid + 1;
    }
    return lo;
}

int main() {
    vector<int> nums = {5, 1, 2, 7, 3, 4};
    cout << splitArray(nums, 3) << "\n"; // expected 8
    return 0;
}
