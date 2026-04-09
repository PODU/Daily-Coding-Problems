// Day 1331: Does any pair of rectangles overlap (full containment counts; edge-touching does not)?
// Convert top_left+dimensions to [xmin,xmax,ymin,ymax]; pairwise strict-interval overlap test. O(n^2).
#include <bits/stdc++.h>
using namespace std;

struct Rect { long long xmin, xmax, ymin, ymax; };

Rect make(long long tlx, long long tly, long long w, long long h) {
    return {tlx, tlx + w, tly - h, tly};
}

bool overlap(const Rect& a, const Rect& b) {
    return a.xmin < b.xmax && b.xmin < a.xmax && a.ymin < b.ymax && b.ymin < a.ymax;
}

int main() {
    vector<Rect> rs = {
        make(1, 4, 3, 3),
        make(-1, 3, 2, 1),
        make(0, 5, 4, 3),
    };
    bool any = false;
    for (size_t i = 0; i < rs.size() && !any; i++)
        for (size_t j = i + 1; j < rs.size(); j++)
            if (overlap(rs[i], rs[j])) { any = true; break; }
    cout << (any ? "true" : "false") << endl; // true
    return 0;
}
