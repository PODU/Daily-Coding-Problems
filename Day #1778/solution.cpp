// Largest sum of non-adjacent numbers via include/exclude DP; empty selection (0) allowed.
// Time: O(N), Space: O(1).
#include <bits/stdc++.h>
using namespace std;

long long maxNonAdjacent(const vector<int>& a) {
    long long incl = 0, excl = 0;
    for (int n : a) {
        long long ni = excl + n;
        long long ne = max(incl, excl);
        incl = ni; excl = ne;
    }
    return max(incl, excl);
}

int main() {
    cout << maxNonAdjacent({2,4,6,2,5}) << "\n";
    cout << maxNonAdjacent({5,1,1,5}) << "\n";
    return 0;
}
