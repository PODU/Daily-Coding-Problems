// Day 735: Find any peak element in O(log N).
// Approach: Binary search - move toward the larger neighbor; a peak must lie that way.
// Time: O(log n), Space: O(1).
#include <bits/stdc++.h>
using namespace std;

int findPeak(vector<int>& a) {
    int lo = 0, hi = a.size() - 1;
    while (lo < hi) {
        int mid = (lo + hi) / 2;
        if (a[mid] < a[mid + 1]) lo = mid + 1;
        else hi = mid;
    }
    return lo;
}

int main() {
    vector<int> a = {0, 2, 5, 3, 1};
    int i = findPeak(a);
    cout << "Peak element: " << a[i] << " (index " << i << ")\n"; // 5 (index 2)
    return 0;
}
