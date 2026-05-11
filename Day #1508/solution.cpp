// Product of array except self without division: prefix-product then suffix-product.
// Two passes, output array only. Time O(n), Space O(1) extra (besides output).
#include <bits/stdc++.h>
using namespace std;

vector<long long> productExceptSelf(const vector<int>& nums) {
    int n = nums.size();
    vector<long long> res(n, 1);
    for (int i = 1; i < n; ++i) res[i] = res[i - 1] * nums[i - 1];
    long long suffix = 1;
    for (int i = n - 1; i >= 0; --i) {
        res[i] *= suffix;
        suffix *= nums[i];
    }
    return res;
}

int main() {
    vector<int> nums = {1, 2, 3, 4, 5};
    vector<long long> res = productExceptSelf(nums);
    cout << "[";
    for (size_t i = 0; i < res.size(); ++i) {
        cout << res[i];
        if (i + 1 < res.size()) cout << ", ";
    }
    cout << "]\n";
    return 0;
}
