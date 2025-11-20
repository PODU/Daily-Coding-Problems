// Day 636: Minimum in a rotated sorted array (no duplicates).
// Approach: binary search comparing mid with right endpoint.
// Time: O(log N), Space: O(1).
#include <bits/stdc++.h>
using namespace std;

int findMin(const vector<int>& a) {
    int lo = 0, hi = a.size() - 1;
    while (lo < hi) {
        int mid = (lo + hi) / 2;
        if (a[mid] > a[hi]) lo = mid + 1;  // min is to the right
        else hi = mid;                     // min is at mid or left
    }
    return a[lo];
}

int main() {
    vector<int> a = {5, 7, 10, 3, 4};
    cout << findMin(a) << "\n"; // 3
    return 0;
}
