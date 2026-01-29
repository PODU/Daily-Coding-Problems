// Longest Increasing Subsequence via patience sorting: maintain "tails" array,
// binary-search the insertion point for each element. Time O(n log n), Space O(n).
#include <bits/stdc++.h>
using namespace std;

int lengthOfLIS(const vector<int>& nums) {
    vector<int> tails; // tails[i] = smallest tail of an increasing subseq of length i+1
    for (int x : nums) {
        auto it = lower_bound(tails.begin(), tails.end(), x);
        if (it == tails.end()) tails.push_back(x);
        else *it = x;
    }
    return (int)tails.size();
}

int main() {
    vector<int> nums = {10, 9, 2, 5, 3, 7, 101, 18};
    cout << "Input: [10, 9, 2, 5, 3, 7, 101, 18]" << endl;
    cout << "LIS length: " << lengthOfLIS(nums) << endl; // 4
    return 0;
}
