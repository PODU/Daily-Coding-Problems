// Day 1429: Find a peak element (greater than both neighbors) in O(log N).
// Approach: binary search; if a[mid] < a[mid+1] a peak lies right, else left/at mid.
// Time: O(log n), Space: O(1).
#include <bits/stdc++.h>
using namespace std;

int findPeak(const vector<int>& a) {
    int lo = 0, hi = a.size() - 1;
    while (lo < hi) {
        int mid = (lo + hi) / 2;
        if (a[mid] < a[mid + 1]) lo = mid + 1;
        else hi = mid;
    }
    return lo; // index of a peak
}

int main() {
    vector<int> a = {1, 3, 5, 7, 6, 4, 2};
    int idx = findPeak(a);
    cout << a[idx] << endl; // 7
    return 0;
}
