// Day 711: Point strictly inside polygon. First reject boundary via on-segment
// test, then ray-casting parity test. Time O(N) per query.
#include <bits/stdc++.h>
using namespace std;

struct P { double x, y; };

bool onSeg(P a, P b, P p) {
    double cross = (b.x - a.x) * (p.y - a.y) - (b.y - a.y) * (p.x - a.x);
    if (fabs(cross) > 1e-9) return false;
    return min(a.x, b.x) - 1e-9 <= p.x && p.x <= max(a.x, b.x) + 1e-9 &&
           min(a.y, b.y) - 1e-9 <= p.y && p.y <= max(a.y, b.y) + 1e-9;
}

bool inside(const vector<P>& poly, P p) {
    int n = poly.size();
    for (int i = 0; i < n; ++i)
        if (onSeg(poly[i], poly[(i + 1) % n], p)) return false; // boundary
    bool in = false;
    for (int i = 0, j = n - 1; i < n; j = i++) {
        if ((poly[i].y > p.y) != (poly[j].y > p.y)) {
            double xint = (poly[j].x - poly[i].x) * (p.y - poly[i].y) /
                          (poly[j].y - poly[i].y) + poly[i].x;
            if (p.x < xint) in = !in;
        }
    }
    return in;
}

int main() {
    vector<P> sq = {{0, 0}, {4, 0}, {4, 4}, {0, 4}};
    cout << (inside(sq, {2, 2}) ? "True" : "False") << endl; // inside
    cout << (inside(sq, {4, 2}) ? "True" : "False") << endl; // boundary
    cout << (inside(sq, {5, 5}) ? "True" : "False") << endl; // outside
    return 0;
}
