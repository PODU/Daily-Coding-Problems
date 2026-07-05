// Trapping Rain Water: two-pointer sweep tracking left/right running maxima.
// Time: O(N), Space: O(1).
#include <bits/stdc++.h>
using namespace std;

long long trap(const vector<int>& h) {
    int l = 0, r = (int)h.size() - 1;
    int leftMax = 0, rightMax = 0;
    long long water = 0;
    while (l < r) {
        if (h[l] < h[r]) {
            leftMax = max(leftMax, h[l]);
            water += leftMax - h[l];
            ++l;
        } else {
            rightMax = max(rightMax, h[r]);
            water += rightMax - h[r];
            --r;
        }
    }
    return water;
}

int main() {
    cout << trap({2, 1, 2}) << "\n";
    cout << trap({3, 0, 1, 3, 0, 5}) << "\n";
    return 0;
}
