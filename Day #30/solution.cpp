// Trapping rain water with two pointers.
// Time: O(n), Space: O(1).
#include <iostream>
#include <vector>
using namespace std;

int trap(const vector<int>& h) {
    int l = 0, r = h.size() - 1;
    int leftMax = 0, rightMax = 0, water = 0;
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
    vector<int> heights = {3, 0, 1, 3, 0, 5};
    cout << trap(heights) << "\n";
    return 0;
}
