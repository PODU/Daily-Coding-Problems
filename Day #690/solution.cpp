// Longest strictly Increasing Subsequence length via patience sorting + binary search.
// Time O(N log N), Space O(N). Also reconstructs one valid LIS.
#include <bits/stdc++.h>
using namespace std;

pair<int, vector<int>> lisLength(const vector<int>& nums) {
    vector<int> tails;        // value of smallest tail per length
    vector<int> tailsIdx;     // index in nums of that tail
    vector<int> prev(nums.size(), -1);
    for (int i = 0; i < (int)nums.size(); i++) {
        int x = nums[i];
        int pos = lower_bound(tails.begin(), tails.end(), x) - tails.begin();
        if (pos == (int)tails.size()) {
            tails.push_back(x);
            tailsIdx.push_back(i);
        } else {
            tails[pos] = x;
            tailsIdx[pos] = i;
        }
        prev[i] = pos > 0 ? tailsIdx[pos - 1] : -1;
    }
    vector<int> seq;
    int k = tailsIdx.empty() ? -1 : tailsIdx.back();
    while (k != -1) {
        seq.push_back(nums[k]);
        k = prev[k];
    }
    reverse(seq.begin(), seq.end());
    return {(int)tails.size(), seq};
}

int main() {
    vector<int> nums = {0, 8, 4, 12, 2, 10, 6, 14, 1, 9, 5, 13, 3, 11, 7, 15};
    pair<int, vector<int> > res = lisLength(nums);
    int len = res.first;
    vector<int> seq = res.second;
    cout << len << endl;
    cout << "[";
    for (size_t i = 0; i < seq.size(); i++) {
        if (i) cout << ", ";
        cout << seq[i];
    }
    cout << "]" << endl;
    return 0;
}
