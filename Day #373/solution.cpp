// Day 373: Longest run of consecutive integers formable from the list.
// Hash set; start counting only at run-starts (x with no x-1 present). O(n) time.
#include <bits/stdc++.h>
using namespace std;

int longestConsecutive(const vector<int>& nums) {
    unordered_set<int> s(nums.begin(), nums.end());
    int best = 0;
    for (int x : s) {
        if (!s.count(x - 1)) {
            int len = 1, cur = x;
            while (s.count(cur + 1)) { cur++; len++; }
            best = max(best, len);
        }
    }
    return best;
}

int main() {
    vector<int> L = {5, 2, 99, 3, 4, 1, 100};
    cout << longestConsecutive(L) << "\n"; // 5
    return 0;
}
