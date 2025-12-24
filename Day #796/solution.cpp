// Day 796: Point strictly inside a polygon.
// Ray-casting (even-odd rule) + on-boundary check. Time O(N), Space O(1).
#include <bits/stdc++.h>
using namespace std;

struct Pt { double x, y; };

// Is point q on segment a-b?
bool onSegment(const Pt& a, const Pt& b, const Pt& q) {
    double cross = (b.x - a.x) * (q.y - a.y) - (b.y - a.y) * (q.x - a.x);
    if (fabs(cross) > 1e-9) return false;
    return min(a.x, b.x) - 1e-9 <= q.x && q.x <= max(a.x, b.x) + 1e-9 &&
           min(a.y, b.y) - 1e-9 <= q.y && q.y <= max(a.y, b.y) + 1e-9;
}

bool insidePolygon(const vector<Pt>& poly, const Pt& p) {
    int n = poly.size();
    // Boundary points count as outside.
    for (int i = 0; i < n; i++)
        if (onSegment(poly[i], poly[(i + 1) % n], p)) return false;
    bool in = false;
    for (int i = 0, j = n - 1; i < n; j = i++) {
        if (((poly[i].y > p.y) != (poly[j].y > p.y)) &&
            (p.x < (poly[j].x - poly[i].x) * (p.y - poly[i].y) /
                       (poly[j].y - poly[i].y) + poly[i].x))
            in = !in;
    }
    return in;
}

int main() {
    vector<Pt> square = {{0, 0}, {4, 0}, {4, 4}, {0, 4}};
    cout << boolalpha;
    cout << insidePolygon(square, {2, 2}) << "\n"; // inside  -> true
    cout << insidePolygon(square, {4, 2}) << "\n"; // boundary -> false
    cout << insidePolygon(square, {5, 5}) << "\n"; // outside -> false
    return 0;
}
