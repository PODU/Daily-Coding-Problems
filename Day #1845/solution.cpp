// Day 1845: Index of nearest larger value (by array distance) via outward expansion.
// Time O(N) per query, Space O(1). Returns -1 if none exists.
#include <bits/stdc++.h>
using namespace std;

int nearestLarger(const vector<int>& a, int i) {
    int n = a.size();
    for (int d = 1; d < n; d++) {
        if (i - d >= 0 && a[i - d] > a[i]) return i - d;
        if (i + d < n && a[i + d] > a[i]) return i + d;
    }
    return -1;  // no larger element
}

int main() {
    cout << nearestLarger({4, 1, 3, 5, 6}, 0) << "\n";  // 3
}
