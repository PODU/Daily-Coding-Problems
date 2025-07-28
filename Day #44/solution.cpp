// Count Inversions via modified merge sort: count cross-pairs while merging.
// Time O(n log n), Space O(n).
#include <bits/stdc++.h>
using namespace std;

long long mergeCount(vector<int>& a, int lo, int hi, vector<int>& tmp) {
    if (hi - lo <= 1) return 0;
    int mid = (lo + hi) / 2;
    long long inv = mergeCount(a, lo, mid, tmp) + mergeCount(a, mid, hi, tmp);
    int i = lo, j = mid, k = lo;
    while (i < mid && j < hi) {
        if (a[i] <= a[j]) tmp[k++] = a[i++];
        else { tmp[k++] = a[j++]; inv += mid - i; }
    }
    while (i < mid) tmp[k++] = a[i++];
    while (j < hi) tmp[k++] = a[j++];
    for (int x = lo; x < hi; x++) a[x] = tmp[x];
    return inv;
}

long long countInversions(vector<int> a) {
    vector<int> tmp(a.size());
    return mergeCount(a, 0, (int)a.size(), tmp);
}

int main() {
    cout << countInversions({2, 4, 1, 3, 5}) << "\n";
    cout << countInversions({5, 4, 3, 2, 1}) << "\n";
    return 0;
}
