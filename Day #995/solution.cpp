// Day 995: Product of array except self, without division.
// Left pass stores prefix products; right pass multiplies by suffix products.
// O(n) time, O(1) extra space (besides output).
#include <bits/stdc++.h>
using namespace std;

vector<long long> products(const vector<long long>& nums) {
    int n = nums.size();
    vector<long long> res(n, 1);
    long long left = 1;
    for (int i = 0; i < n; ++i) { res[i] = left; left *= nums[i]; }
    long long right = 1;
    for (int i = n - 1; i >= 0; --i) { res[i] *= right; right *= nums[i]; }
    return res;
}

void print(const vector<long long>& v) {
    cout << '[';
    for (size_t i = 0; i < v.size(); ++i) cout << v[i] << (i + 1 < v.size() ? ", " : "");
    cout << "]\n";
}

int main() {
    print(products({1, 2, 3, 4, 5})); // [120, 60, 40, 30, 24]
    print(products({3, 2, 1}));        // [2, 3, 6]
    return 0;
}
