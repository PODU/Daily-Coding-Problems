// Day 1751: Min steps to visit points in order on an 8-directional grid.
// Sum of Chebyshev distances max(|dx|,|dy|) between consecutive points. O(n) time, O(1) space.
#include <bits/stdc++.h>
using namespace std;

int minSteps(const vector<pair<int,int>>& pts) {
    int total = 0;
    for (size_t i = 1; i < pts.size(); ++i) {
        int dx = abs(pts[i].first  - pts[i-1].first);
        int dy = abs(pts[i].second - pts[i-1].second);
        total += max(dx, dy);
    }
    return total;
}

int main() {
    vector<pair<int,int>> pts = {{0,0},{1,1},{1,2}};
    cout << minSteps(pts) << "\n"; // 2
    return 0;
}
