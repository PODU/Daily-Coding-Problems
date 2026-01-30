// Day 987: Nearest larger number index.
// Expand outward from i checking i-d and i+d; first larger wins. O(n) per query, O(1) space.
// Follow-up: with O(n^2) preprocessing (or sparse tables) each query can be answered in O(1).
#include <bits/stdc++.h>
using namespace std;

// Returns index of nearest larger element, or -1 if none.
int nearestLarger(const vector<int>& a, int i) {
    int n = a.size();
    for (int d = 1; d < n; d++) {
        int l = i - d, r = i + d;
        if (l >= 0 && a[l] > a[i]) return l; // prefer left on ties
        if (r < n && a[r] > a[i]) return r;
    }
    return -1;
}

int main() {
    vector<int> a = {4, 1, 3, 5, 6};
    int idx = nearestLarger(a, 0);
    if (idx == -1) cout << "null" << endl;
    else cout << idx << endl; // expected 3
    return 0;
}
