// Day 564: Largest sum of non-adjacent numbers (empty subset allowed -> answer >= 0).
// DP tracking incl/excl running maxes. Time O(n), Space O(1).
#include <bits/stdc++.h>
using namespace std;

long long largestNonAdjacent(const vector<long long>& nums) {
    long long incl = 0, excl = 0; // best sums including / excluding previous element
    for (long long x : nums) {
        long long newIncl = excl + x;
        excl = max(incl, excl);
        incl = newIncl;
    }
    return max({incl, excl, 0LL});
}

int main() {
    cout << largestNonAdjacent({2, 4, 6, 2, 5}) << "\n";
    cout << largestNonAdjacent({5, 1, 1, 5}) << "\n";
    return 0;
}
