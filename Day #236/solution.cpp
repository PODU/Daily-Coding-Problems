// Point in polygon: ray-casting (even-odd rule). Boundary points are detected separately
// and return false. Time: O(N), Space: O(1).
#include <bits/stdc++.h>
using namespace std;

bool onSegment(double px, double py, double ax, double ay, double bx, double by) {
    double cross = (bx - ax) * (py - ay) - (by - ay) * (px - ax);
    if (fabs(cross) > 1e-9) return false;
    return min(ax, bx) - 1e-9 <= px && px <= max(ax, bx) + 1e-9 &&
           min(ay, by) - 1e-9 <= py && py <= max(ay, by) + 1e-9;
}

bool inside(vector<pair<double, double>>& poly, double px, double py) {
    int n = poly.size();
    bool in = false;
    for (int i = 0, j = n - 1; i < n; j = i++) {
        double xi = poly[i].first, yi = poly[i].second;
        double xj = poly[j].first, yj = poly[j].second;
        if (onSegment(px, py, xi, yi, xj, yj)) return false; // boundary
        bool intersect = ((yi > py) != (yj > py)) &&
                         (px < (xj - xi) * (py - yi) / (yj - yi) + xi);
        if (intersect) in = !in;
    }
    return in;
}

int main() {
    vector<pair<double, double>> poly = {{0, 0}, {4, 0}, {4, 4}, {0, 4}};
    cout << boolalpha;
    cout << inside(poly, 2, 2) << "\n"; // true (inside)
    cout << inside(poly, 4, 2) << "\n"; // false (on boundary)
    cout << inside(poly, 5, 5) << "\n"; // false (outside)
}
