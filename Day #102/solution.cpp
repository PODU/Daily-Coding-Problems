// Day 102: Contiguous subarray summing to K via prefix sums + hashmap. For each
// prefix p look for p-K seen earlier; earliest-ending match. O(n) time.
#include <bits/stdc++.h>
using namespace std;

vector<int> subarraySum(vector<int>& nums, int k) {
    unordered_map<long long, int> first{{0, -1}};
    long long prefix = 0;
    for (int j = 0; j < (int)nums.size(); j++) {
        prefix += nums[j];
        auto it = first.find(prefix - k);
        if (it != first.end())
            return vector<int>(nums.begin() + it->second + 1, nums.begin() + j + 1);
        first.emplace(prefix, j); // keep earliest
    }
    return {};
}

int main() {
    vector<int> nums = {1, 2, 3, 4, 5};
    auto r = subarraySum(nums, 9);
    cout << "[";
    for (size_t i = 0; i < r.size(); i++) cout << r[i] << (i + 1 < r.size() ? ", " : "");
    cout << "]\n"; // [2, 3, 4]
    return 0;
}
