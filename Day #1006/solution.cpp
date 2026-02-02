// Count intersecting segment pairs: sort by p, count inversions in q via merge sort.
// Time O(n log n), Space O(n).
#include <bits/stdc++.h>
using namespace std;

long long mergeCount(vector<int>& a, vector<int>& tmp, int l, int r) {
    if (r - l <= 1) return 0;
    int m = (l + r) / 2;
    long long cnt = mergeCount(a, tmp, l, m) + mergeCount(a, tmp, m, r);
    int i = l, j = m, k = l;
    while (i < m && j < r) {
        if (a[i] <= a[j]) tmp[k++] = a[i++];
        else { tmp[k++] = a[j++]; cnt += m - i; }
    }
    while (i < m) tmp[k++] = a[i++];
    while (j < r) tmp[k++] = a[j++];
    for (int t = l; t < r; ++t) a[t] = tmp[t];
    return cnt;
}

long long countIntersections(vector<int> p, vector<int> q) {
    int n = p.size();
    vector<int> idx(n);
    iota(idx.begin(), idx.end(), 0);
    sort(idx.begin(), idx.end(), [&](int x, int y){ return p[x] < p[y]; });
    vector<int> qs(n);
    for (int i = 0; i < n; ++i) qs[i] = q[idx[i]];
    vector<int> tmp(n);
    return mergeCount(qs, tmp, 0, n);
}

int main() {
    vector<int> p = {1, 2, 3, 4};
    vector<int> q = {4, 3, 2, 1};
    cout << countIntersections(p, q) << endl; // 6
    return 0;
}
