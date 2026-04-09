// Day 1329: Closest pair of points via divide & conquer. O(n log n) time.
// Sort by x, recurse on halves, then check the middle strip ordered by y (each point vs next ~7).
#include <bits/stdc++.h>
using namespace std;

struct P { long long x, y; };

double dist(const P& a, const P& b) {
    double dx = a.x - b.x, dy = a.y - b.y;
    return sqrt(dx * dx + dy * dy);
}

double rec(vector<P>& px, vector<P>& py, P& ra, P& rb) {
    int n = px.size();
    if (n <= 3) {
        double best = 1e18;
        for (int i = 0; i < n; i++)
            for (int j = i + 1; j < n; j++) {
                double d = dist(px[i], px[j]);
                if (d < best) { best = d; ra = px[i]; rb = px[j]; }
            }
        return best;
    }
    int mid = n / 2;
    P pivot = px[mid];
    long long midX = pivot.x;
    vector<P> lx(px.begin(), px.begin() + mid), rx(px.begin() + mid, px.end());
    vector<P> ly, ry;
    for (auto& p : py) {
        if (p.x < pivot.x || (p.x == pivot.x && p.y < pivot.y)) ly.push_back(p);
        else ry.push_back(p);
    }

    P a1, b1, a2, b2;
    double dl = rec(lx, ly, a1, b1);
    double dr = rec(rx, ry, a2, b2);
    double best = dl;
    if (dl <= dr) { ra = a1; rb = b1; } else { best = dr; ra = a2; rb = b2; }

    vector<P> strip;
    for (auto& p : py) if (llabs(p.x - midX) < best) strip.push_back(p);
    for (size_t i = 0; i < strip.size(); i++)
        for (size_t j = i + 1; j < strip.size() && (strip[j].y - strip[i].y) < best; j++) {
            double d = dist(strip[i], strip[j]);
            if (d < best) { best = d; ra = strip[i]; rb = strip[j]; }
        }
    return best;
}

int main() {
    vector<P> pts = {{1,1},{-1,-1},{3,4},{6,1},{-1,-6},{-4,-3}};
    vector<P> px = pts, py = pts;
    sort(px.begin(), px.end(), [](const P&a, const P&b){ return a.x < b.x || (a.x==b.x && a.y<b.y); });
    sort(py.begin(), py.end(), [](const P&a, const P&b){ return a.y < b.y; });
    P a, b;
    rec(px, py, a, b);
    if (a.x > b.x || (a.x == b.x && a.y > b.y)) swap(a, b);
    cout << "[(" << a.x << ", " << a.y << "), (" << b.x << ", " << b.y << ")]" << endl;
    // [(-1, -1), (1, 1)]
    return 0;
}
