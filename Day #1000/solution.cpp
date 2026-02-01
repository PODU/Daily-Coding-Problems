// Day 1000: Minimum of a rotated sorted array (no duplicates).
// Binary search comparing mid with the right end. O(log N) time, O(1) space.
#include <bits/stdc++.h>
using namespace std;

int findMin(const vector<int>& nums) {
    int lo = 0, hi = nums.size() - 1;
    while (lo < hi) {
        int mid = (lo + hi) / 2;
        if (nums[mid] > nums[hi]) lo = mid + 1;
        else hi = mid;
    }
    return nums[lo];
}

int main() {
    cout << findMin({5, 7, 10, 3, 4}) << "\n"; // 3
    return 0;
}
