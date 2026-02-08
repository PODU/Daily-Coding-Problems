// Trapping rain water via two pointers tracking leftMax/rightMax. Time O(N), Space O(1).
#include <bits/stdc++.h>
using namespace std;

int trap(const vector<int>& h) {
    int l = 0, r = (int)h.size() - 1, lm = 0, rm = 0, water = 0;
    while (l < r) {
        if (h[l] < h[r]) {
            lm = max(lm, h[l]);
            water += lm - h[l];
            l++;
        } else {
            rm = max(rm, h[r]);
            water += rm - h[r];
            r--;
        }
    }
    return water;
}

int main() {
    cout << "[2, 1, 2] -> " << trap({2, 1, 2}) << "\n";
    cout << "[3, 0, 1, 3, 0, 5] -> " << trap({3, 0, 1, 3, 0, 5}) << "\n";
    return 0;
}
