// Day 185: Area of intersection of two axis-aligned rectangles (top-left + width/height, y up).
// Overlap = max(0, dx) * max(0, dy). Time O(1), Space O(1).
#include <bits/stdc++.h>
using namespace std;

struct Rect { int left, top, w, h; };

long long intersectionArea(const Rect& a, const Rect& b) {
    int ox = min(a.left + a.w, b.left + b.w) - max(a.left, b.left);
    int oy = min(a.top, b.top) - max(a.top - a.h, b.top - b.h);
    if (ox <= 0 || oy <= 0) return 0;
    return (long long)ox * oy;
}

int main() {
    Rect a{1, 4, 3, 3};   // top_left (1,4), 3x3
    Rect b{0, 5, 4, 3};   // top_left (0,5), 4x3
    cout << intersectionArea(a, b) << "\n";
    return 0;
}
