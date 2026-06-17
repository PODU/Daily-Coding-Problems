// Day 1680: Strict point-in-polygon. Reject boundary via on-segment test, else
// ray-casting parity. Time O(N), Space O(1).
#include <bits/stdc++.h>
using namespace std;

bool onSeg(double x1, double y1, double x2, double y2, double px, double py) {
    double cross = (x2 - x1) * (py - y1) - (y2 - y1) * (px - x1);
    if (fabs(cross) > 1e-9) return false;
    return px >= min(x1, x2) - 1e-9 && px <= max(x1, x2) + 1e-9 &&
           py >= min(y1, y2) - 1e-9 && py <= max(y1, y2) + 1e-9;
}

bool inside(const vector<pair<double,double>>& poly, double px, double py) {
    int n = poly.size();
    for (int i = 0; i < n; ++i) {
        double x1 = poly[i].first, y1 = poly[i].second;
        double x2 = poly[(i + 1) % n].first, y2 = poly[(i + 1) % n].second;
        if (onSeg(x1, y1, x2, y2, px, py)) return false; // boundary
    }
    bool in = false;
    for (int i = 0, j = n - 1; i < n; j = i++) {
        double xi = poly[i].first, yi = poly[i].second;
        double xj = poly[j].first, yj = poly[j].second;
        if ((yi > py) != (yj > py) &&
            px < (xj - xi) * (py - yi) / (yj - yi) + xi)
            in = !in;
    }
    return in;
}

int main() {
    vector<pair<double,double>> sq = {{0,0},{4,0},{4,4},{0,4}};
    cout << (inside(sq, 2, 2) ? "True" : "False") << "\n"; // True
    cout << (inside(sq, 4, 2) ? "True" : "False") << "\n"; // False (boundary)
    cout << (inside(sq, 5, 5) ? "True" : "False") << "\n"; // False (outside)
    return 0;
}
