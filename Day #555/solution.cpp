// Search in rotated sorted array via modified binary search. O(log n) time, O(1) space.
#include <bits/stdc++.h>
using namespace std;

int search(const vector<int>& a, int target) {
    int lo = 0, hi = (int)a.size() - 1;
    while (lo <= hi) {
        int mid = lo + (hi - lo) / 2;
        if (a[mid] == target) return mid;
        if (a[lo] <= a[mid]) { // left half sorted
            if (a[lo] <= target && target < a[mid]) hi = mid - 1;
            else lo = mid + 1;
        } else { // right half sorted
            if (a[mid] < target && target <= a[hi]) lo = mid + 1;
            else hi = mid - 1;
        }
    }
    return -1;
}

int main() {
    vector<int> arr = {13, 18, 25, 2, 8, 10};
    int idx = search(arr, 8);
    cout << idx << "\n";
    int miss = search(arr, 7);
    cout << (miss == -1 ? "null" : to_string(miss)) << "\n";
    return 0;
}
