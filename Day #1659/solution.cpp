// Smallest unsorted window. Scan L->R tracking max for end, R->L tracking min for start. O(n) time, O(1) space.
#include <iostream>
#include <vector>
#include <climits>
using namespace std;

pair<int,int> unsortedWindow(const vector<int>& a) {
    int n = a.size(), end = -1, start = -1;
    int mx = INT_MIN, mn = INT_MAX;
    for (int i = 0; i < n; ++i) {
        mx = max(mx, a[i]);
        if (a[i] < mx) end = i;
    }
    for (int i = n - 1; i >= 0; --i) {
        mn = min(mn, a[i]);
        if (a[i] > mn) start = i;
    }
    return {start, end};
}

int main() {
    auto r = unsortedWindow({3, 7, 5, 6, 9});
    cout << "(" << r.first << ", " << r.second << ")\n";
    return 0;
}
