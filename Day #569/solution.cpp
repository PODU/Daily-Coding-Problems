// Find min and max using ~3*ceil(N/2) comparisons (pairwise method).
// Time: O(N) with <2N comparisons. Space: O(1).
#include <bits/stdc++.h>
using namespace std;

pair<int,int> minMax(const vector<int>& a, long long& cmps) {
    int n = (int)a.size();
    int mn, mx, i;
    if (n & 1) { mn = mx = a[0]; i = 1; }            // odd: seed with first
    else {                                            // even: seed with first pair
        cmps++;
        if (a[0] < a[1]) { mn = a[0]; mx = a[1]; }
        else { mn = a[1]; mx = a[0]; }
        i = 2;
    }
    for (; i + 1 < n; i += 2) {
        int lo, hi;
        cmps++;
        if (a[i] < a[i+1]) { lo = a[i]; hi = a[i+1]; }
        else { lo = a[i+1]; hi = a[i]; }
        cmps++; if (lo < mn) mn = lo;
        cmps++; if (hi > mx) mx = hi;
    }
    return {mn, mx};
}

int main() {
    vector<int> a = {7, 2, 9, 4, 1, 8, 3};
    long long cmps = 0;
    auto r = minMax(a, cmps);
    cout << "min=" << r.first << " max=" << r.second << "\n";
    return 0;
}
