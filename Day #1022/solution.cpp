// Day 1022: Single number where all others appear 3 times.
// Approach: ones/twos bitmask accumulators. Time O(N), Space O(1).
#include <bits/stdc++.h>
using namespace std;

int singleNumber(const vector<int>& nums) {
    int ones = 0, twos = 0;
    for (int x : nums) {
        ones = (ones ^ x) & ~twos;
        twos = (twos ^ x) & ~ones;
    }
    return ones;
}

int main() {
    vector<vector<int>> tests = {{6, 1, 3, 3, 3, 6, 6}, {13, 19, 13, 13}};
    for (auto& t : tests) cout << singleNumber(t) << "\n";
}
