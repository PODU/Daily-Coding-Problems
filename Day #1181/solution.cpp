// Day 1181: Find the minimum in a rotated sorted array (no duplicates).
// Binary search: compare mid to the right end to discard the sorted half.
// Time O(log N), Space O(1).
#include <bits/stdc++.h>
using namespace std;

int findMin(const vector<int>& a) {
    int lo = 0, hi = (int)a.size() - 1;
    while (lo < hi) {
        int mid = lo + (hi - lo) / 2;
        if (a[mid] > a[hi]) lo = mid + 1;   // min is to the right
        else hi = mid;                       // min is at mid or to the left
    }
    return a[lo];
}

int main() {
    cout << findMin({5, 7, 10, 3, 4}) << "\n"; // 3
    return 0;
}
