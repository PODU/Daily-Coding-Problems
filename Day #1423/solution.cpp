// Day 1423: can you reach the end of the array (each value = max steps forward)?
// Approach: greedy, track farthest reachable index. O(n) time, O(1) space.
#include <bits/stdc++.h>
using namespace std;

bool canReachEnd(const vector<int>& nums) {
    int farthest = 0, n = (int)nums.size();
    for (int i = 0; i < n; ++i) {
        if (i > farthest) return false;
        farthest = max(farthest, i + nums[i]);
    }
    return true;
}

int main() {
    cout << (canReachEnd({1, 3, 1, 2, 0, 1}) ? "true" : "false") << "\n"; // true
    cout << (canReachEnd({1, 2, 1, 0, 0}) ? "true" : "false") << "\n";    // false
    return 0;
}
