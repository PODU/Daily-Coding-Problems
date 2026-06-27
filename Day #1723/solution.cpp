// Day 1723: Search element in rotated sorted array.
// Modified binary search: one half is always sorted; decide which side to recurse.
// Time: O(log n), Space: O(1). Returns index or -1 if absent.
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
    vector<int> a = {13, 18, 25, 2, 8, 10};
    for (int target : {8, 99}) {
        int i = search(a, target);
        // the problem statement asks for null when the element is absent
        if (i >= 0) cout << i << "\n"; // 4
        else cout << "null\n";
    }
    return 0;
}
