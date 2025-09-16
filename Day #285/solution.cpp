// Day 285: Count buildings (east->west) with a sunset (west) view.
// Single backward pass tracking running max. Time O(N), Space O(1).
#include <bits/stdc++.h>
using namespace std;

int sunsetViews(const vector<int>& h) {
    int count = 0, maxSoFar = INT_MIN;
    for (int i = (int)h.size() - 1; i >= 0; i--) { // from west end inward
        if (h[i] > maxSoFar) { count++; maxSoFar = h[i]; }
    }
    return count;
}

int main() {
    cout << sunsetViews({3, 7, 8, 3, 6, 1}) << "\n"; // 3
    return 0;
}
