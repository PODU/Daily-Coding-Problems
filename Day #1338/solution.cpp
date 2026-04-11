// Longest consecutive run via hash set: start only at run heads (x-1 absent), walk up. O(n) time, O(n) space.
#include <bits/stdc++.h>
using namespace std;

int longestConsecutive(const vector<int>& nums) {
    unordered_set<int> s(nums.begin(), nums.end());
    int best = 0;
    for (int x : s) {
        if (s.count(x - 1)) continue; // not a run head
        int cur = x, len = 1;
        while (s.count(cur + 1)) { ++cur; ++len; }
        best = max(best, len);
    }
    return best;
}

int main() {
    vector<int> nums = {100, 4, 200, 1, 3, 2};
    cout << longestConsecutive(nums) << "\n";
    return 0;
}
