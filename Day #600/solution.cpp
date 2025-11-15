// Day 600: Closest pair of points on a plane.
// Approach: divide & conquer with a y-sorted strip check. Time O(n log n), Space O(n).
#include <bits/stdc++.h>
using namespace std;

struct P { long long x, y; };

double dist2(const P& a, const P& b) {
    double dx = a.x - b.x, dy = a.y - b.y;
    return dx * dx + dy * dy;
}

pair<double, pair<P, P>> rec(vector<P>& px, int lo, int hi) {
    int n = hi - lo;
    double best = 1e18;
    pair<P, P> bp;
    if (n <= 3) {
        for (int i = lo; i < hi; i++)
            for (int j = i + 1; j < hi; j++) {
                double d = dist2(px[i], px[j]);
                if (d < best) { best = d; bp = {px[i], px[j]}; }
            }
        return {best, bp};
    }
    int mid = (lo + hi) / 2;
    long long midx = px[mid].x;
    auto L = rec(px, lo, mid);
    auto R = rec(px, mid, hi);
    auto cur = L.first <= R.first ? L : R;
    best = cur.first; bp = cur.second;
    double dd = sqrt(best);

    vector<P> strip;
    for (int i = lo; i < hi; i++)
        if (fabs((double)px[i].x - midx) < dd) strip.push_back(px[i]);
    sort(strip.begin(), strip.end(), [](const P& a, const P& b) { return a.y < b.y; });
    for (size_t i = 0; i < strip.size(); i++)
        for (size_t j = i + 1; j < strip.size() && (strip[j].y - strip[i].y) < dd; j++) {
            double d = dist2(strip[i], strip[j]);
            if (d < best) { best = d; bp = {strip[i], strip[j]}; dd = sqrt(best); }
        }
    return {best, bp};
}

int main() {
    vector<P> pts = {{1, 1}, {-1, -1}, {3, 4}, {6, 1}, {-1, -6}, {-4, -3}};
    sort(pts.begin(), pts.end(), [](const P& a, const P& b) {
        return a.x != b.x ? a.x < b.x : a.y < b.y;
    });
    auto res = rec(pts, 0, pts.size());
    P a = res.second.first, b = res.second.second;
    if (a.x > b.x || (a.x == b.x && a.y > b.y)) swap(a, b);
    cout << "[(" << a.x << ", " << a.y << "), (" << b.x << ", " << b.y << ")]\n";
    return 0;
}
