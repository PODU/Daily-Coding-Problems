// Rectangle intersection area: O(1) time, O(1) space.
// top_left is top y, height extends downward. x_overlap*y_overlap clamped at 0.
#include <bits/stdc++.h>
using namespace std;

struct Rect { double left, top, width, height; };

double intersectArea(const Rect& a, const Rect& b) {
    double aRight = a.left + a.width, bRight = b.left + b.width;
    double aBottom = a.top - a.height, bBottom = b.top - b.height;
    double xo = max(0.0, min(aRight, bRight) - max(a.left, b.left));
    double yo = max(0.0, min(a.top, b.top) - max(aBottom, bBottom));
    return xo * yo;
}

int main() {
    Rect r1{1, 4, 3, 3};
    Rect r2{0, 5, 4, 3};
    printf("%g\n", intersectArea(r1, r2));
    return 0;
}
