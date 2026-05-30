// Day 1581: Area of intersection of two axis-aligned rectangles.
// top_left=(x,y), dims=(w,h); x-range [x,x+w], y-range [y-h,y]. Overlap = clamped widths.
// Time: O(1); Space: O(1).
#include <bits/stdc++.h>
using namespace std;

long long intersectionArea(int x1, int y1, int w1, int h1, int x2, int y2, int w2, int h2) {
    long long ow = min(x1 + w1, x2 + w2) - max(x1, x2);
    long long oh = min(y1, y2) - max(y1 - h1, y2 - h2);
    if (ow <= 0 || oh <= 0) return 0;
    return ow * oh;
}

int main() {
    // {top_left:(1,4), w=3,h=3} and {top_left:(0,5), w=4,h=3}
    cout << intersectionArea(1, 4, 3, 3, 0, 5, 4, 3) << "\n"; // 6
    return 0;
}
