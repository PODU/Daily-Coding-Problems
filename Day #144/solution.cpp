// Nearest larger element's index by index-distance: expand outward from i. O(n) per query, O(1) space.
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
    cout << nearestLarger(a, 0) << endl; // 3
    return 0;
}
