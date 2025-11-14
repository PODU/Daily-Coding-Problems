// Day 597: Detect a Pythagorean triplet a^2 + b^2 = c^2 in an array.
// Approach: square values, sort, fix c as the largest, two-pointer. Time O(n^2), Space O(n).
#include <bits/stdc++.h>
using namespace std;

bool hasPythagoreanTriplet(vector<long long> nums) {
    for (auto& v : nums) v = v * v;
    sort(nums.begin(), nums.end());
    int n = nums.size();
    for (int c = n - 1; c >= 2; c--) {
        int lo = 0, hi = c - 1;
        while (lo < hi) {
            long long s = nums[lo] + nums[hi];
            if (s == nums[c]) return true;
            else if (s < nums[c]) lo++;
            else hi--;
        }
    }
    return false;
}

int main() {
    vector<long long> arr = {3, 1, 4, 6, 5};   // contains 3,4,5
    cout << (hasPythagoreanTriplet(arr) ? "true" : "false") << "\n";
    return 0;
}
