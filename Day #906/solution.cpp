// Closest pair of points via divide & conquer: sort by x, recurse, merge with strip check by y.
// O(n log n) time, O(n) space.
#include <bits/stdc++.h>
using namespace std;

typedef pair<long long,long long> Pt;

double dist(const Pt& a, const Pt& b) {
    double dx = a.first - b.first, dy = a.second - b.second;
    return sqrt(dx*dx + dy*dy);
}

double rec(vector<Pt>& px, vector<Pt>& py, int lo, int hi, Pt& ba, Pt& bb) {
    int n = hi - lo;
    if (n <= 3) {
        double best = 1e18;
        for (int i = lo; i < hi; i++)
            for (int j = i+1; j < hi; j++) {
                double d = dist(px[i], px[j]);
                if (d < best) { best = d; ba = px[i]; bb = px[j]; }
            }
        sort(px.begin()+lo, px.begin()+hi, [](const Pt&a,const Pt&b){return a.second<b.second;});
        return best;
    }
    int mid = lo + n/2;
    long long midx = px[mid].first;
    double dl = rec(px, py, lo, mid, ba, bb);
    Pt ra, rb;
    double dr = rec(px, py, mid, hi, ra, rb);
    double best = dl;
    if (dr < best) { best = dr; ba = ra; bb = rb; }
    // px[lo..mid) and px[mid..hi) are each sorted by y now; merge into py-like
    vector<Pt> merged;
    merge(px.begin()+lo, px.begin()+mid, px.begin()+mid, px.begin()+hi,
          back_inserter(merged), [](const Pt&a,const Pt&b){return a.second<b.second;});
    // strip
    vector<Pt> strip;
    for (auto& p : merged) if (llabs(p.first - midx) < best) strip.push_back(p);
    for (int i = 0; i < (int)strip.size(); i++)
        for (int j = i+1; j < (int)strip.size() && (strip[j].second - strip[i].second) < best; j++) {
            double d = dist(strip[i], strip[j]);
            if (d < best) { best = d; ba = strip[i]; bb = strip[j]; }
        }
    for (int i = 0; i < (int)merged.size(); i++) px[lo+i] = merged[i];
    return best;
}

int main() {
    vector<Pt> pts = {{1,1},{-1,-1},{3,4},{6,1},{-1,-6},{-4,-3}};
    sort(pts.begin(), pts.end());
    vector<Pt> py = pts;
    Pt a, b;
    rec(pts, py, 0, pts.size(), a, b);
    if (b < a) swap(a, b);
    cout << "[(" << a.first << ", " << a.second << "), ("
         << b.first << ", " << b.second << ")]\n";
    return 0;
}
