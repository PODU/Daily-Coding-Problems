// Count inversions using modified merge sort (count cross-pairs during merge).
// Time: O(N log N), Space: O(N).
#include <iostream>
#include <vector>
using namespace std;

long long mergeCount(vector<int>& a, vector<int>& tmp, int lo, int hi) {
    if (hi - lo <= 1) return 0;
    int mid = lo + (hi - lo) / 2;
    long long inv = mergeCount(a, tmp, lo, mid) + mergeCount(a, tmp, mid, hi);
    int i = lo, j = mid, k = lo;
    while (i < mid && j < hi) {
        if (a[i] <= a[j]) tmp[k++] = a[i++];
        else { tmp[k++] = a[j++]; inv += (mid - i); }
    }
    while (i < mid) tmp[k++] = a[i++];
    while (j < hi) tmp[k++] = a[j++];
    for (int t = lo; t < hi; t++) a[t] = tmp[t];
    return inv;
}

long long countInversions(vector<int> a) {
    vector<int> tmp(a.size());
    return mergeCount(a, tmp, 0, (int)a.size());
}

int main() {
    cout << "[2, 4, 1, 3, 5] has " << countInversions({2, 4, 1, 3, 5}) << " inversions\n";
    cout << "[5, 4, 3, 2, 1] has " << countInversions({5, 4, 3, 2, 1}) << " inversions\n";
    return 0;
}
