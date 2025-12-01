// Count inversions via modified merge sort. Time O(N log N), Space O(N).
#include <bits/stdc++.h>
using namespace std;

long long mergeCount(vector<int>& a, int l, int r) {
    if (r - l <= 1) return 0;
    int m = (l + r) / 2;
    long long inv = mergeCount(a, l, m) + mergeCount(a, m, r);
    vector<int> tmp;
    tmp.reserve(r - l);
    int i = l, j = m;
    while (i < m && j < r) {
        if (a[i] <= a[j]) tmp.push_back(a[i++]);
        else { tmp.push_back(a[j++]); inv += (m - i); }
    }
    while (i < m) tmp.push_back(a[i++]);
    while (j < r) tmp.push_back(a[j++]);
    for (int k = 0; k < (int)tmp.size(); ++k) a[l + k] = tmp[k];
    return inv;
}

long long countInversions(vector<int> a) { return mergeCount(a, 0, a.size()); }

int main() {
    cout << countInversions({2, 4, 1, 3, 5}) << "\n";
    cout << countInversions({5, 4, 3, 2, 1}) << "\n";
    return 0;
}
