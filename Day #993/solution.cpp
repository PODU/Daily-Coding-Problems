// Day 993: Majority element (the value occurring more than floor(n/2) times).
// Count occurrences in a hash map and return the most frequent value. This also
// yields the expected answer for the README's (non-strict) example. O(n) time/space.
#include <bits/stdc++.h>
using namespace std;

int majority(const vector<int>& nums) {
    unordered_map<int, int> freq;
    int best = nums[0], bestCount = 0;
    for (int x : nums) {
        int c = ++freq[x];
        if (c > bestCount) { bestCount = c; best = x; }
    }
    return best;
}

int main() {
    cout << majority({1, 2, 1, 1, 3, 4, 0}) << "\n"; // 1
    return 0;
}
