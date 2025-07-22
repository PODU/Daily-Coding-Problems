// Max sum of non-adjacent numbers: track best-including vs best-excluding current.
// Time: O(n), Space: O(1). (Empty pick allowed, so negatives can be skipped.)
#include <bits/stdc++.h>
using namespace std;

long long maxNonAdjacent(const vector<int>& nums) {
    long long incl = 0, excl = 0;
    for (int n : nums) {
        long long ni = excl + n;
        long long ne = max(incl, excl);
        incl = ni;
        excl = ne;
    }
    return max(incl, excl);
}

int main() {
    cout << maxNonAdjacent({2, 4, 6, 2, 5}) << "\n";
    return 0;
}
