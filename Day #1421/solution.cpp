// Day 1421: product of all elements except self, without division.
// Approach: prefix products pass then suffix products pass. O(n) time, O(1) extra (besides output).
#include <bits/stdc++.h>
using namespace std;

vector<long long> productExceptSelf(const vector<int>& nums) {
    int n = nums.size();
    vector<long long> res(n, 1);
    long long prefix = 1;
    for (int i = 0; i < n; ++i) { res[i] = prefix; prefix *= nums[i]; }
    long long suffix = 1;
    for (int i = n - 1; i >= 0; --i) { res[i] *= suffix; suffix *= nums[i]; }
    return res;
}

void show(const vector<int>& in) {
    auto r = productExceptSelf(in);
    cout << "[";
    for (size_t i = 0; i < r.size(); ++i) cout << r[i] << (i + 1 < r.size() ? ", " : "");
    cout << "]\n";
}

int main() {
    show({1, 2, 3, 4, 5}); // [120, 60, 40, 30, 24]
    show({3, 2, 1});       // [2, 3, 6]
    return 0;
}
