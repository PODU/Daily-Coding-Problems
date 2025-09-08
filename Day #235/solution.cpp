// Min & Max in ~3N/2 comparisons: process elements in pairs, compare the pair first,
// then smaller vs min and larger vs max. Time: O(N), Space: O(1). Comparisons ~ 3*ceil(N/2)-2.
#include <bits/stdc++.h>
using namespace std;

pair<int, int> minMax(const vector<int>& a) {
    int n = a.size();
    int mn, mx, i;
    if (n % 2 == 0) {
        if (a[0] < a[1]) { mn = a[0]; mx = a[1]; }
        else { mn = a[1]; mx = a[0]; }
        i = 2;
    } else {
        mn = mx = a[0];
        i = 1;
    }
    while (i < n) {
        int x = a[i], y = a[i + 1];
        if (x < y) {
            mn = min(mn, x);
            mx = max(mx, y);
        } else {
            mn = min(mn, y);
            mx = max(mx, x);
        }
        i += 2;
    }
    return {mn, mx};
}

int main() {
    vector<int> a = {3, 5, 1, 2, 4, 8, 7};
    auto r = minMax(a);
    cout << "min=" << r.first << " max=" << r.second << "\n"; // min=1 max=8
}
