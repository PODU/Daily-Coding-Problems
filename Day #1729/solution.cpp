// Day 1729: Count buildings with a sunset (west) view.
// Single right-to-left pass tracking max height seen to the west; a building is
// visible iff strictly taller than all to its west. Time: O(n). Space: O(1).
#include <bits/stdc++.h>
using namespace std;

int countSunsetViews(const vector<int>& heights) {
    int count = 0, maxWest = 0;
    // Scan from the west end (rightmost index) toward the east.
    for (int i = (int)heights.size() - 1; i >= 0; --i) {
        if (heights[i] > maxWest) {
            ++count;
            maxWest = heights[i];
        }
    }
    return count;
}

int main() {
    vector<int> heights = {3, 7, 8, 3, 6, 1}; // east -> west
    cout << countSunsetViews(heights) << "\n"; // 1, 6, 8 are visible => 3
    return 0;
}
