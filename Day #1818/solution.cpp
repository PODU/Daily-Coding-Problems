// Closest pair of points via divide-and-conquer on x, strip-merge on y.
// Each point tagged with an id for an unambiguous left/right split. Time: O(n log n). Space: O(n).
#include <bits/stdc++.h>
using namespace std;

struct Pt { long long x, y; int id; };

double dist2(const Pt& a, const Pt& b) {
    double dx = a.x - b.x, dy = a.y - b.y;
    return dx*dx + dy*dy;
}

pair<Pt,Pt> rec(vector<Pt>& px, vector<Pt>& py) {
    int n = px.size();
    if (n <= 3) {
        double best = 1e18; pair<Pt,Pt> bp{px[0], px[0]};
        for (int i = 0; i < n; i++)
            for (int j = i+1; j < n; j++)
                if (dist2(px[i], px[j]) < best) { best = dist2(px[i], px[j]); bp = {px[i], px[j]}; }
        return bp;
    }
    int mid = n / 2;
    long long midX = px[mid].x;
    vector<Pt> lx(px.begin(), px.begin()+mid), rx(px.begin()+mid, px.end());
    set<int> leftIds;
    for (auto& p : lx) leftIds.insert(p.id);
    vector<Pt> ly, ry;
    for (auto& p : py) (leftIds.count(p.id) ? ly : ry).push_back(p);

    auto bl = rec(lx, ly);
    auto br = rec(rx, ry);
    pair<Pt,Pt> best = dist2(bl.first,bl.second) < dist2(br.first,br.second) ? bl : br;
    double d2 = dist2(best.first, best.second);

    vector<Pt> strip;
    for (auto& p : py) { double dx = p.x - midX; if (dx*dx < d2) strip.push_back(p); }
    for (size_t i = 0; i < strip.size(); i++)
        for (size_t j = i+1; j < strip.size(); j++) {
            double dy = strip[j].y - strip[i].y;
            if (dy*dy >= d2) break;
            if (dist2(strip[i], strip[j]) < d2) { d2 = dist2(strip[i], strip[j]); best = {strip[i], strip[j]}; }
        }
    return best;
}

pair<Pt,Pt> closestPair(vector<Pt> pts) {
    vector<Pt> px = pts, py = pts;
    sort(px.begin(), px.end(), [](const Pt&a,const Pt&b){return a.x<b.x || (a.x==b.x && a.y<b.y);});
    sort(py.begin(), py.end(), [](const Pt&a,const Pt&b){return a.y<b.y || (a.y==b.y && a.x<b.x);});
    return rec(px, py);
}

int main() {
    vector<vector<long long>> raw = {{1,1},{-1,-1},{3,4},{6,1},{-1,-6},{-4,-3}};
    vector<Pt> pts;
    for (int i = 0; i < (int)raw.size(); i++) pts.push_back({raw[i][0], raw[i][1], i});
    auto r = closestPair(pts);
    Pt a = r.first, b = r.second;
    if (a.x > b.x || (a.x == b.x && a.y > b.y)) swap(a, b);
    cout << "[(" << a.x << ", " << a.y << "), (" << b.x << ", " << b.y << ")]\n";
    // [(-1, -1), (1, 1)]
    return 0;
}
