// Day 1578: Find min and max using ~3N/2 comparisons (pairwise method).
// Compare elements in pairs, then each pair contributes one compare to min and one to max.
// Time: O(N) with ceil(3N/2)-2 comparisons; Space: O(1).
#include <bits/stdc++.h>
using namespace std;

pair<int,int> minMax(const vector<int>& a) {
    int n = a.size();
    int mn, mx, i;
    if (n & 1) { mn = mx = a[0]; i = 1; }
    else { if (a[0] < a[1]) { mn = a[0]; mx = a[1]; } else { mn = a[1]; mx = a[0]; } i = 2; }
    for (; i + 1 < n; i += 2) {
        int lo, hi;
        if (a[i] < a[i + 1]) { lo = a[i]; hi = a[i + 1]; }   // 1 compare per pair
        else { lo = a[i + 1]; hi = a[i]; }
        if (lo < mn) mn = lo;                                 // 1 compare
        if (hi > mx) mx = hi;                                 // 1 compare
    }
    return {mn, mx};
}

int main() {
    vector<int> a = {3, 5, 1, 2, 8, 7, 4};
    auto r = minMax(a);
    cout << "min=" << r.first << " max=" << r.second << "\n"; // min=1 max=8
    return 0;
}
