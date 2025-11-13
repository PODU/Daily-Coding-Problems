// Day 593: Count buildings with a view of the setting sun (west).
// Array is east->west (index 0 = east). A building sees the sunset iff it is
// taller than every building further west (higher index). Single right-to-left pass.
// Time: O(n), Space: O(1).
#include <bits/stdc++.h>
using namespace std;

int countSunsetViews(const vector<int>& h) {
    int count = 0, maxSeen = INT_MIN;
    for (int i = (int)h.size() - 1; i >= 0; --i) { // from west end
        if (h[i] > maxSeen) { ++count; maxSeen = h[i]; }
    }
    return count;
}

int main() {
    vector<int> h = {3, 7, 8, 3, 6, 1};
    cout << countSunsetViews(h) << endl; // 3 (buildings 1, 6, 8)
    return 0;
}
