// Longest Increasing Subsequence (length), patience sorting.
// Maintain tails[]; binary-search insertion point for each value. O(n log n) time, O(n) space.
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
    cout << lengthOfLIS(nums) << "\n";
    return 0;
}
