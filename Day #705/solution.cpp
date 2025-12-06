// Day 705: Trapping rain water.
// Approach: two pointers tracking running left/right maxima; the smaller side is
// bounded so we can resolve it. Time O(N), Space O(1).
#include <bits/stdc++.h>
using namespace std;

long long trap(const vector<int>& h) {
    int l = 0, r = (int)h.size() - 1, lmax = 0, rmax = 0;
    long long water = 0;
    while (l < r) {
        if (h[l] < h[r]) {
            lmax = max(lmax, h[l]); water += lmax - h[l]; ++l;
        } else {
            rmax = max(rmax, h[r]); water += rmax - h[r]; --r;
        }
    }
    return water;
}

int main() {
    cout << trap({2, 1, 2}) << "\n";          // 1
    cout << trap({3, 0, 1, 3, 0, 5}) << "\n"; // 8
    return 0;
}
