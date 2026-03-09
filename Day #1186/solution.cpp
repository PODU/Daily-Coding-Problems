// Count intersecting segment pairs: sort segments by p, then count inversions in q.
// Two segments cross iff their p-order and q-order disagree. O(n log n) time, O(n) space.
#include <bits/stdc++.h>
using namespace std;

static long long mergeCount(vector<int>& a, int l, int r) {
    if (r - l <= 1) return 0;
    int m = (l + r) / 2;
    long long c = mergeCount(a, l, m) + mergeCount(a, m, r);
    vector<int> tmp;
    int i = l, j = m;
    while (i < m && j < r) {
        if (a[i] <= a[j]) tmp.push_back(a[i++]);
        else { tmp.push_back(a[j++]); c += m - i; }
    }
    while (i < m) tmp.push_back(a[i++]);
    while (j < r) tmp.push_back(a[j++]);
    for (int k = l; k < r; k++) a[k] = tmp[k - l];
    return c;
}

long long countIntersections(vector<int> p, vector<int> q) {
    int n = p.size();
    vector<int> idx(n);
    iota(idx.begin(), idx.end(), 0);
    sort(idx.begin(), idx.end(), [&](int x, int y) { return p[x] < p[y]; });
    vector<int> qq(n);
    for (int k = 0; k < n; k++) qq[k] = q[idx[k]];
    return mergeCount(qq, 0, n);
}

int main() {
    vector<int> p = {1, 2, 3}, q = {3, 1, 2};
    cout << countIntersections(p, q) << endl; // 2
}
