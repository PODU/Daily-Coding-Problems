// Day 953: largest sum of non-adjacent numbers (may pick none -> >= 0).
// incl/excl DP. Time O(n), Space O(1).
#include <bits/stdc++.h>
using namespace std;

long long maxNonAdjacent(const vector<long long>& a) {
    long long incl = 0, excl = 0;
    for (long long x : a) {
        long long ni = excl + x;
        long long ne = max(incl, excl);
        incl = ni; excl = ne;
    }
    return max(incl, excl);
}

int main() {
    cout << maxNonAdjacent({2, 4, 6, 2, 5}) << "\n"; // 13
    cout << maxNonAdjacent({5, 1, 1, 5}) << "\n";    // 10
    return 0;
}
