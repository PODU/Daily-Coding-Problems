// Day 1718: Count intersecting segment pairs (p_i on y=0 -> q_i on y=1).
// Two segments cross iff their (p, q) ordering is inverted: sort by p,
// count inversions in q via merge sort. Time: O(n log n), Space: O(n).
#include <bits/stdc++.h>
using namespace std;

long long mergeCount(vector<int>& a, int l, int r) {
    if (r - l <= 1) return 0;
    int mid = (l + r) / 2;
    long long inv = mergeCount(a, l, mid) + mergeCount(a, mid, r);
    vector<int> tmp;
    int i = l, j = mid;
    while (i < mid && j < r) {
        if (a[i] <= a[j]) tmp.push_back(a[i++]);
        else { inv += mid - i; tmp.push_back(a[j++]); }
    }
    while (i < mid) tmp.push_back(a[i++]);
    while (j < r) tmp.push_back(a[j++]);
    for (int t = 0; t < (int)tmp.size(); ++t) a[l + t] = tmp[t];
    return inv;
}

long long countIntersections(vector<int> p, vector<int> q) {
    int n = (int)p.size();
    vector<int> idx(n);
    iota(idx.begin(), idx.end(), 0);
    sort(idx.begin(), idx.end(), [&](int x, int y){ return p[x] < p[y]; });
    vector<int> qs(n);
    for (int i = 0; i < n; ++i) qs[i] = q[idx[i]];
    return mergeCount(qs, 0, n);
}

int main() {
    vector<int> p = {1, 2, 3, 4};
    vector<int> q = {2, 1, 4, 3};
    cout << "Intersecting pairs: " << countIntersections(p, q) << "\n";
    return 0;
}
