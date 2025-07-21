// Product of array except self via prefix and suffix passes (no division).
// Time: O(n), Space: O(1) extra (excluding output).
#include <bits/stdc++.h>
using namespace std;

vector<long long> productExceptSelf(const vector<int>& nums) {
    int n = nums.size();
    vector<long long> res(n, 1);
    long long pre = 1;
    for (int i = 0; i < n; i++) { res[i] = pre; pre *= nums[i]; }
    long long suf = 1;
    for (int i = n - 1; i >= 0; i--) { res[i] *= suf; suf *= nums[i]; }
    return res;
}

int main() {
    vector<int> nums = {1, 2, 3, 4, 5};
    auto res = productExceptSelf(nums);
    cout << "[";
    for (size_t i = 0; i < res.size(); i++) cout << res[i] << (i + 1 < res.size() ? ", " : "");
    cout << "]\n";
    return 0;
}
