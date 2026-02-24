// Day 1120 - Minimum steps to cover points in order (8-directional moves)
// Cost between two points is Chebyshev distance max(|dx|,|dy|); sum them.
// Time: O(n), Space: O(1).
#include <bits/stdc++.h>
using namespace std;

int minSteps(const vector<pair<int,int>>& pts) {
    int total = 0;
    for (size_t i = 1; i < pts.size(); ++i)
        total += max(abs(pts[i].first - pts[i-1].first),
                     abs(pts[i].second - pts[i-1].second));
    return total;
}

int main() {
    vector<pair<int,int>> points = {{0, 0}, {1, 1}, {1, 2}};
    cout << minSteps(points) << endl; // 2
    return 0;
}
