// Approach: sort segments by p, then count inversions in the q-order (BIT/merge sort).
// Two segments cross iff their p-order and q-order disagree => an inversion. Time O(n log n), Space O(n).
#include <bits/stdc++.h>
using namespace std;

long long mergeCount(vector<int>& a, int l, int r) {
    if (r - l <= 1) return 0;
    int m = (l + r) / 2;
    long long inv = mergeCount(a, l, m) + mergeCount(a, m, r);
    vector<int> tmp;
    int i = l, j = m;
    while (i < m && j < r) {
        if (a[i] <= a[j]) tmp.push_back(a[i++]);
        else { tmp.push_back(a[j++]); inv += m - i; }
    }
    while (i < m) tmp.push_back(a[i++]);
    while (j < r) tmp.push_back(a[j++]);
    for (int k = 0; k < (int)tmp.size(); ++k) a[l + k] = tmp[k];
    return inv;
}

long long countIntersections(vector<int> p, vector<int> q) {
    int n = p.size();
    vector<int> idx(n);
    iota(idx.begin(), idx.end(), 0);
    sort(idx.begin(), idx.end(), [&](int x, int y){ return p[x] < p[y]; });
    vector<int> qs(n);
    for (int i = 0; i < n; ++i) qs[i] = q[idx[i]];
    return mergeCount(qs, 0, n);
}

int main() {
    vector<int> p = {1, 2, 3, 4};
    vector<int> q = {4, 3, 2, 1};
    cout << countIntersections(p, q) << "\n";
    return 0;
}
