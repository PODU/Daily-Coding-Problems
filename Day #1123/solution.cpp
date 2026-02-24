// Day 1123 - Largest divisible subset
// Sort, LIS-style DP where j extends i if a[i] % a[j] == 0; reconstruct via
// parent pointers. Time: O(n^2), Space: O(n).
#include <bits/stdc++.h>
using namespace std;

vector<int> largestDivisibleSubset(vector<int> nums) {
    if (nums.empty()) return {};
    sort(nums.begin(), nums.end());
    int n = nums.size();
    vector<int> dp(n, 1), parent(n, -1);
    int best = 0;
    for (int i = 0; i < n; ++i) {
        for (int j = 0; j < i; ++j)
            if (nums[i] % nums[j] == 0 && dp[j] + 1 > dp[i]) {
                dp[i] = dp[j] + 1;
                parent[i] = j;
            }
        if (dp[i] > dp[best]) best = i;
    }
    vector<int> res;
    for (int k = best; k != -1; k = parent[k]) res.push_back(nums[k]);
    reverse(res.begin(), res.end());
    return res;
}

void printVec(const vector<int>& v) {
    cout << "[";
    for (size_t i = 0; i < v.size(); ++i)
        cout << v[i] << (i + 1 < v.size() ? ", " : "");
    cout << "]\n";
}

int main() {
    printVec(largestDivisibleSubset({3, 5, 10, 20, 21})); // [5, 10, 20]
    printVec(largestDivisibleSubset({1, 3, 6, 24}));       // [1, 3, 6, 24]
    return 0;
}
