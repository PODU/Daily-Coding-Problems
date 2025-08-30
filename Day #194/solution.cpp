// Day 194: Segments p_i->q_i cross iff order of p and q disagree. Count = inversions of q
// after sorting pairs by p. Merge-sort inversion count. Time O(n log n), Space O(n).
#include <bits/stdc++.h>
using namespace std;

long long mergeCount(vector<int>& v, int l, int r) {
    if (r - l <= 1) return 0;
    int m = (l + r) / 2;
    long long cnt = mergeCount(v, l, m) + mergeCount(v, m, r);
    vector<int> tmp; int i = l, j = m;
    while (i < m && j < r) {
        if (v[i] <= v[j]) tmp.push_back(v[i++]);
        else { tmp.push_back(v[j++]); cnt += m - i; }
    }
    while (i < m) tmp.push_back(v[i++]);
    while (j < r) tmp.push_back(v[j++]);
    copy(tmp.begin(), tmp.end(), v.begin() + l);
    return cnt;
}

long long countCrossings(vector<int> p, vector<int> q) {
    int n = p.size();
    vector<int> idx(n);
    iota(idx.begin(), idx.end(), 0);
    sort(idx.begin(), idx.end(), [&](int a, int b){ return p[a] < p[b]; });
    vector<int> qs(n);
    for (int k = 0; k < n; k++) qs[k] = q[idx[k]];
    return mergeCount(qs, 0, n);
}

int main() {
    cout << countCrossings({1, 2, 3, 4}, {4, 3, 2, 1}) << "\n";
    return 0;
}
