// Fixed point in sorted distinct array via binary search (nums[i]-i non-decreasing).
// Time: O(log n), Space: O(1).
#include <bits/stdc++.h>
using namespace std;

int fixedPoint(const vector<int>& nums) {
    int lo = 0, hi = (int)nums.size() - 1, ans = -1;
    while (lo <= hi) {
        int mid = lo + (hi - lo) / 2;
        if (nums[mid] == mid) { ans = mid; break; }
        else if (nums[mid] < mid) lo = mid + 1;
        else hi = mid - 1;
    }
    return ans;
}

int main() {
    vector<int> a = {-6, 0, 2, 40};
    vector<int> b = {1, 5, 7, 8};
    int r1 = fixedPoint(a);
    if (r1 == -1) cout << "False\n"; else cout << r1 << "\n";
    int r2 = fixedPoint(b);
    if (r2 == -1) cout << "False\n"; else cout << r2 << "\n";
    return 0;
}
