// Day 1449: Trapping Rain Water. Two-pointer sweep tracking running left/right
// maxima. Time O(n), Space O(1).
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
            l++;
        } else {
            rightMax = max(rightMax, h[r]);
            water += rightMax - h[r];
            r--;
        }
    }
    return water;
}

int main() {
    cout << trap({2,1,2}) << "\n";          // 1
    cout << trap({3,0,1,3,0,5}) << "\n";    // 8
    return 0;
}
