// Sorted squares via two-pointer merge from both ends, filling result from the back.
// Time: O(n), Space: O(n).
#include <bits/stdc++.h>
using namespace std;

vector<long long> sortedSquares(const vector<long long>& nums) {
    int n = nums.size();
    vector<long long> res(n);
    int lo = 0, hi = n - 1;
    for (int k = n - 1; k >= 0; --k) {
        long long l = nums[lo] * nums[lo];
        long long r = nums[hi] * nums[hi];
        if (l > r) { res[k] = l; lo++; }
        else       { res[k] = r; hi--; }
    }
    return res;
}

int main() {
    vector<long long> nums = {-9, -2, 0, 2, 3};
    vector<long long> res = sortedSquares(nums);
    cout << "[";
    for (size_t i = 0; i < res.size(); ++i) {
        cout << res[i];
        if (i + 1 < res.size()) cout << ", ";
    }
    cout << "]\n";
    return 0;
}
