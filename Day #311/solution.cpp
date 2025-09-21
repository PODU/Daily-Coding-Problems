// Day 311: Find a peak (greater than both neighbors) when ends are the lowest.
// Binary search toward the rising side. O(log N).
#include <bits/stdc++.h>
using namespace std;
int findPeak(vector<int>& a) {
    int lo = 0, hi = a.size() - 1;
    while (lo < hi) {
        int mid = (lo + hi) / 2;
        if (a[mid] < a[mid + 1]) lo = mid + 1; else hi = mid;
    }
    return lo;
}
int main() {
    vector<int> a = {1, 3, 5, 7, 6, 4, 2};
    int p = findPeak(a);
    cout << "index " << p << " value " << a[p] << "\n"; // index 3 value 7
}
