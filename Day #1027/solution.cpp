// Day 1027: Longest consecutive elements sequence.
// Hash set; only count runs starting at numbers with no predecessor. Time O(n), Space O(n).
#include <bits/stdc++.h>
using namespace std;

int longestConsecutive(const vector<int>& nums) {
    unordered_set<int> s(nums.begin(), nums.end());
    int best = 0;
    for (int x : s) {
        if (!s.count(x - 1)) {
            int len = 1, cur = x;
            while (s.count(cur + 1)) { ++cur; ++len; }
            best = max(best, len);
        }
    }
    return best;
}

int main() {
    vector<int> nums = {100, 4, 200, 1, 3, 2};
    cout << longestConsecutive(nums) << "\n";
}
