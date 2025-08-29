// Day 187: Do any two rectangles overlap (containment counts; edge-touching does not).
// Pairwise interior-overlap test. Time O(n^2), Space O(1). (Sweep line gives O(n log n).)
#include <bits/stdc++.h>
using namespace std;

struct Rect { int left, top, w, h; };

bool overlap(const Rect& a, const Rect& b) {
    int ox = min(a.left + a.w, b.left + b.w) - max(a.left, b.left);
    int oy = min(a.top, b.top) - max(a.top - a.h, b.top - b.h);
    return ox > 0 && oy > 0;
}

bool anyOverlap(const vector<Rect>& rs) {
    for (size_t i = 0; i < rs.size(); i++)
        for (size_t j = i + 1; j < rs.size(); j++)
            if (overlap(rs[i], rs[j])) return true;
    return false;
}

int main() {
    vector<Rect> rs = {{1, 4, 3, 3}, {-1, 3, 2, 1}, {0, 5, 4, 3}};
    cout << boolalpha << anyOverlap(rs) << "\n";
    return 0;
}
