// Day 1290: Strict point-in-polygon test (ray casting), boundary counts as outside.
// Check edges for on-boundary, then parity of rightward ray crossings. Time O(n), Space O(1).
#include <bits/stdc++.h>
using namespace std;

struct Pt { double x, y; };

bool onSegment(Pt a, Pt b, Pt p) {
    double cross = (b.x - a.x) * (p.y - a.y) - (b.y - a.y) * (p.x - a.x);
    if (fabs(cross) > 1e-9) return false;
    return p.x >= min(a.x, b.x) - 1e-9 && p.x <= max(a.x, b.x) + 1e-9 &&
           p.y >= min(a.y, b.y) - 1e-9 && p.y <= max(a.y, b.y) + 1e-9;
}

bool inside(const vector<Pt>& poly, Pt p) {
    int n = poly.size();
    for (int i = 0; i < n; ++i)
        if (onSegment(poly[i], poly[(i + 1) % n], p)) return false; // boundary
    bool res = false;
    for (int i = 0, j = n - 1; i < n; j = i++) {
        double xi = poly[i].x, yi = poly[i].y, xj = poly[j].x, yj = poly[j].y;
        if ((yi > p.y) != (yj > p.y)) {
            double xint = (xj - xi) * (p.y - yi) / (yj - yi) + xi;
            if (p.x < xint) res = !res;
        }
    }
    return res;
}

int main() {
    vector<Pt> square = {{0, 0}, {4, 0}, {4, 4}, {0, 4}};
    cout << (inside(square, {2, 2}) ? "True" : "False") << "\n"; // True
    cout << (inside(square, {5, 5}) ? "True" : "False") << "\n"; // False
    cout << (inside(square, {4, 2}) ? "True" : "False") << "\n"; // False (boundary)
    return 0;
}
