// Day 374: Lowest index i with arr[i]==i in a sorted, distinct-int array.
// f(i)=arr[i]-i is non-decreasing (distinct & sorted), so binary-search the
// leftmost i with f(i)>=0 and check equality. Time O(log n), Space O(1).
#include <bits/stdc++.h>
using namespace std;

int fixedPoint(const vector<int>& arr) {
    int lo = 0, hi = (int)arr.size() - 1, ans = -1;
    while (lo <= hi) {
        int mid = lo + (hi - lo) / 2;
        if (arr[mid] >= mid) { ans = mid; hi = mid - 1; }
        else lo = mid + 1;
    }
    return (ans != -1 && arr[ans] == ans) ? ans : -1; // -1 means null
}

int main() {
    vector<int> arr = {-5, -3, 2, 3};
    int r = fixedPoint(arr);
    cout << (r == -1 ? "null" : to_string(r)) << "\n"; // 2
    return 0;
}
