// Search in rotated sorted array via modified binary search.
// Time: O(log n), Space: O(1).
#include <bits/stdc++.h>
using namespace std;

int searchRotated(const vector<int>& a, int target) {
    int lo = 0, hi = (int)a.size() - 1;
    while (lo <= hi) {
        int mid = lo + (hi - lo) / 2;
        if (a[mid] == target) return mid;
        if (a[lo] <= a[mid]) {            // left half sorted
            if (a[lo] <= target && target < a[mid]) hi = mid - 1;
            else lo = mid + 1;
        } else {                          // right half sorted
            if (a[mid] < target && target <= a[hi]) lo = mid + 1;
            else hi = mid - 1;
        }
    }
    return -1; // null
}

int main() {
    vector<int> a = {13, 18, 25, 2, 8, 10};
    int idx = searchRotated(a, 8);
    if (idx < 0) cout << "null" << endl;
    else cout << idx << endl; // 4
    return 0;
}
