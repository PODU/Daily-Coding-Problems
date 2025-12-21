// Day 773: Count inversions via modified merge sort. O(n log n) time, O(n) space.
#include <bits/stdc++.h>
using namespace std;

long long mergeCount(vector<int>& a, int l, int r, vector<int>& tmp) {
    if (r - l <= 1) return 0;
    int m = (l + r) / 2;
    long long cnt = mergeCount(a, l, m, tmp) + mergeCount(a, m, r, tmp);
    int i = l, j = m, k = l;
    while (i < m && j < r) {
        if (a[i] <= a[j]) tmp[k++] = a[i++];
        else { tmp[k++] = a[j++]; cnt += m - i; }
    }
    while (i < m) tmp[k++] = a[i++];
    while (j < r) tmp[k++] = a[j++];
    for (int t = l; t < r; t++) a[t] = tmp[t];
    return cnt;
}

long long countInversions(vector<int> a) {
    vector<int> tmp(a.size());
    return mergeCount(a, 0, a.size(), tmp);
}

int main() {
    cout << countInversions({2, 4, 1, 3, 5}) << endl; // 3
    cout << countInversions({5, 4, 3, 2, 1}) << endl; // 10
    return 0;
}
