// Find a peak in a rotated array (ends lowest) via binary search. O(log N) time, O(1) space.
#include <bits/stdc++.h>
using namespace std;

int findPeak(const vector<int>& a) {
    int lo = 0, hi = a.size() - 1;
    while (lo < hi) {
        int mid = (lo + hi) / 2;
        if (a[mid] < a[mid + 1]) lo = mid + 1;
        else hi = mid;
    }
    return a[lo];
}

int main() {
    vector<int> arr = {5, 7, 9, 3, 1};
    cout << findPeak(arr) << endl;
    return 0;
}
