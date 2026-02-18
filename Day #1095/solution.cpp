// Detect if any pair of axis-aligned rectangles overlap (containment counts). Time O(n^2), Space O(n).
#include <bits/stdc++.h>
using namespace std;

struct Rect { double minx, maxx, miny, maxy; };

Rect fromTopLeft(double x, double y, double w, double h) {
    return {x, x + w, y - h, y}; // top_left, width grows right, height grows down
}

bool overlap(const Rect& a, const Rect& b) {
    return a.minx < b.maxx && b.minx < a.maxx && a.miny < b.maxy && b.miny < a.maxy;
}

bool anyOverlap(const vector<Rect>& rs) {
    for (size_t i = 0; i < rs.size(); i++)
        for (size_t j = i + 1; j < rs.size(); j++)
            if (overlap(rs[i], rs[j])) return true;
    return false;
}

int main() {
    vector<Rect> rs = {
        fromTopLeft(1, 4, 3, 3),
        fromTopLeft(-1, 3, 2, 1),
        fromTopLeft(0, 5, 4, 3)};
    cout << (anyOverlap(rs) ? "true" : "false") << "\n";
}
