// Day 1437: Length of longest strictly increasing subsequence.
// Approach: Patience sorting; maintain tails array, binary search lower_bound.
// Time: O(n log n), Space: O(n).
#include <bits/stdc++.h>
using namespace std;

int lengthOfLIS(const vector<int>& nums) {
    vector<int> tails;
    for (int x : nums) {
        auto it = lower_bound(tails.begin(), tails.end(), x);
        if (it == tails.end()) tails.push_back(x);
        else *it = x;
    }
    return (int)tails.size();
}

int main() {
    vector<int> nums = {0, 8, 4, 12, 2, 10, 6, 14, 1, 9, 5, 13, 3, 11, 7, 15};
    cout << lengthOfLIS(nums) << endl; // 6
    return 0;
}
