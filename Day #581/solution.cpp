// Rectangle intersection area via overlap of x and y ranges. Time O(1), Space O(1).
#include <iostream>
#include <algorithm>
using namespace std;

int intersectionArea(int tlx1, int tly1, int w1, int h1,
                     int tlx2, int tly2, int w2, int h2) {
    int left1 = tlx1, right1 = tlx1 + w1, top1 = tly1, bottom1 = tly1 - h1;
    int left2 = tlx2, right2 = tlx2 + w2, top2 = tly2, bottom2 = tly2 - h2;
    int overlapW = min(right1, right2) - max(left1, left2);
    int overlapH = min(top1, top2) - max(bottom1, bottom2);
    return max(0, overlapW) * max(0, overlapH);
}

int main() {
    cout << intersectionArea(1, 4, 3, 3, 0, 5, 4, 3) << endl;
    return 0;
}
