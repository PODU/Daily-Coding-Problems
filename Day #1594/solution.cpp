// Approach: Pairwise O(n^2) overlap check on axis-aligned rectangles (strict, positive-area).
// Time O(n^2), Space O(1). Sweep line possible for large n; pairwise is fine here.
#include <bits/stdc++.h>
using namespace std;

struct Rect { double x1, y1, x2, y2; }; // x1<x2, y1<y2

bool overlap(const Rect& a, const Rect& b) {
    // strict overlap => positive area intersection
    return a.x1 < b.x2 && b.x1 < a.x2 && a.y1 < b.y2 && b.y1 < a.y2;
}

bool anyOverlap(const vector<Rect>& rs) {
    for (size_t i = 0; i < rs.size(); ++i)
        for (size_t j = i + 1; j < rs.size(); ++j)
            if (overlap(rs[i], rs[j])) return true;
    return false;
}

// Build from top_left (x,y) and dims (w,h): x in [x,x+w], y in [y-h,y]
Rect make(double x, double y, double w, double h) { return { x, y - h, x + w, y }; }

int main() {
    vector<Rect> rs = {
        make(1, 4, 3, 3),   // x[1,4] y[1,4]
        make(-1, 3, 2, 1),  // x[-1,1] y[2,3]
        make(0, 5, 4, 3)    // x[0,4] y[2,5]
    };
    cout << (anyOverlap(rs) ? "true" : "false") << "\n";
    return 0;
}
