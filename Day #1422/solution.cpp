// Day 1422: largest sum of non-adjacent numbers (values may be 0/negative).
// Approach: rolling include/exclude DP; empty subset allowed (floor 0). O(n) time, O(1) space.
#include <bits/stdc++.h>
using namespace std;

long long maxNonAdjacent(const vector<int>& nums) {
    long long incl = 0, excl = 0;
    for (int n : nums) {
        long long newIncl = excl + n;
        long long newExcl = max(incl, excl);
        incl = newIncl;
        excl = newExcl;
    }
    return max(incl, excl);
}

int main() {
    cout << maxNonAdjacent({2, 4, 6, 2, 5}) << "\n"; // 13
    cout << maxNonAdjacent({5, 1, 1, 5}) << "\n";    // 10
    return 0;
}
