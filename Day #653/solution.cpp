// Brute-force all O(k^2) pairs; rectangles overlap iff their projections strictly overlap on both axes.
// Sweep-line O(k log k) is possible but k^2 is clear. Time O(k^2), space O(k).
#include <bits/stdc++.h>
using namespace std;

struct Rect { int x1, y1, x2, y2; };

bool overlap(const Rect& a, const Rect& b) {
    return a.x1 < b.x2 && b.x1 < a.x2 && a.y1 < b.y2 && b.y1 < a.y2;
}

// build from top_left (x,y) and dimensions (w,h): x1=x, x2=x+w, y2=y(top), y1=y-h(bottom)
Rect make(int x, int y, int w, int h) { return {x, y - h, x + w, y}; }

int main() {
    vector<Rect> rects = {
        make(1, 4, 3, 3),   // R1
        make(-1, 3, 2, 1),  // R2
        make(0, 5, 4, 3)    // R3
    };
    bool any = false;
    for (size_t i = 0; i < rects.size() && !any; ++i)
        for (size_t j = i + 1; j < rects.size(); ++j)
            if (overlap(rects[i], rects[j])) { any = true; break; }
    cout << (any ? "true" : "false") << "\n";
    return 0;
}
