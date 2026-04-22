// Single pass from the west end (array right), tracking the tallest seen so far;
// a building has a view iff it is taller than everything to its west.
// Time O(n), Space O(1).
#include <bits/stdc++.h>
using namespace std;

int countSunsetViews(const vector<int>& h) {
    int count = 0, maxW = INT_MIN;
    for (int i = (int)h.size() - 1; i >= 0; i--) {
        if (h[i] > maxW) { count++; maxW = h[i]; }
    }
    return count;
}

int main() {
    cout << countSunsetViews({3, 7, 8, 3, 6, 1}) << "\n"; // 3
    return 0;
}
