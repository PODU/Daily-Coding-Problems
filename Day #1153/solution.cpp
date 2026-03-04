// Day 1153: Min steps to visit points in order (8-directional moves).
// Sum of Chebyshev distances max(|dx|,|dy|) between consecutive points. O(n) time, O(1) space.
#include <bits/stdc++.h>
using namespace std;

int minSteps(const vector<pair<int,int>>& pts) {
    int total = 0;
    for (size_t i = 1; i < pts.size(); ++i)
        total += max(abs(pts[i].first - pts[i-1].first), abs(pts[i].second - pts[i-1].second));
    return total;
}

int main() {
    vector<pair<int,int>> pts = {{0, 0}, {1, 1}, {1, 2}};
    cout << minSteps(pts) << "\n"; // 2
    return 0;
}
