// Longest consecutive sequence: hash set, count runs from each sequence start. O(n) time, O(n) space.
#include <bits/stdc++.h>
using namespace std;

int longestConsecutive(const vector<int>& nums) {
    unordered_set<int> s(nums.begin(), nums.end());
    int best = 0;
    for (int n : s) {
        if (!s.count(n - 1)) {
            int cur = n, len = 1;
            while (s.count(cur + 1)) { ++cur; ++len; }
            best = max(best, len);
        }
    }
    return best;
}

int main() {
    vector<int> nums = {100, 4, 200, 1, 3, 2};
    cout << longestConsecutive(nums) << "\n";
    return 0;
}
