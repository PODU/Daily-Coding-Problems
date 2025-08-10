// Day 99: Longest consecutive sequence. Hash all values; begin counting only at
// sequence starts (n-1 absent) and walk up. O(n) time, O(n) space.
#include <bits/stdc++.h>
using namespace std;

int longestConsecutive(vector<int>& nums) {
    unordered_set<int> s(nums.begin(), nums.end());
    int best = 0;
    for (int n : s) {
        if (!s.count(n - 1)) {
            int length = 1;
            while (s.count(n + length)) length++;
            best = max(best, length);
        }
    }
    return best;
}

int main() {
    vector<int> nums = {100, 4, 200, 1, 3, 2};
    cout << longestConsecutive(nums) << "\n"; // 4
    return 0;
}
