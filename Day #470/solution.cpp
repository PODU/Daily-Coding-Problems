// Nearest larger: expand outward from i, return first index with greater value.
// Time: O(n), Space: O(1).
#include <bits/stdc++.h>
using namespace std;

int nearestLarger(const vector<int>& a, int i) {
    int n = a.size();
    for (int d = 1; d < n; d++) {
        if (i - d >= 0 && a[i - d] > a[i]) return i - d;
        if (i + d < n && a[i + d] > a[i]) return i + d;
    }
    return -1; // null
}

int main() {
    vector<int> a = {4, 1, 3, 5, 6};
    int r = nearestLarger(a, 0);
    if (r == -1) cout << "null" << endl;
    else cout << r << endl;
    return 0;
}
