// Day 768: Find min and max together using ~3N/2 comparisons (< 2*(N-2)).
// Process elements in pairs: compare the pair, then smaller vs min, larger vs max.
#include <bits/stdc++.h>
using namespace std;

pair<int,int> minMax(const vector<int>& a) {
    int n = a.size();
    int mn, mx, i;
    if (n % 2 == 0) { mn = min(a[0], a[1]); mx = max(a[0], a[1]); i = 2; }
    else { mn = mx = a[0]; i = 1; }
    for (; i < n; i += 2) {
        int lo = a[i], hi = a[i + 1];
        if (lo > hi) swap(lo, hi);          // 1 comparison
        mn = min(mn, lo);                    // 1 comparison
        mx = max(mx, hi);                    // 1 comparison
    }
    return {mn, mx};
}

int main() {
    vector<int> a = {3, 5, 1, 2, 4, 8, 7};
    pair<int,int> r = minMax(a);
    cout << "min=" << r.first << " max=" << r.second << endl; // min=1 max=8
    return 0;
}
