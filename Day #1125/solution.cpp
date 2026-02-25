// Day 1125 - Contiguous sublist summing to K
// Prefix sums + hash map (handles negatives) to find a range with sum == K in
// one pass. Time: O(n), Space: O(n).
#include <bits/stdc++.h>
using namespace std;

vector<int> subarraySum(const vector<int>& nums, int k) {
    unordered_map<long long,int> seen;
    seen[0] = -1;
    long long total = 0;
    for (int j = 0; j < (int)nums.size(); ++j) {
        total += nums[j];
        if (seen.count(total - k)) {
            int i = seen[total - k];
            return vector<int>(nums.begin() + i + 1, nums.begin() + j + 1);
        }
        if (!seen.count(total)) seen[total] = j;
    }
    return {};
}

int main() {
    auto res = subarraySum({1, 2, 3, 4, 5}, 9);
    cout << "[";
    for (size_t i = 0; i < res.size(); ++i)
        cout << res[i] << (i + 1 < res.size() ? ", " : "");
    cout << "]\n"; // [2, 3, 4]
    return 0;
}
