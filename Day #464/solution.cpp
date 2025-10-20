// Day 464: Largest divisible subset.
// Approach: sort, then LIS-style DP where dp[i]=longest chain ending at i;
// j divides i means a[i]%a[j]==0. Reconstruct via parent pointers.
// Time: O(n^2), Space: O(n).
#include <iostream>
#include <vector>
#include <algorithm>
using namespace std;

vector<int> largestDivisibleSubset(vector<int> nums) {
    if (nums.empty()) return {};
    sort(nums.begin(), nums.end());
    int n = nums.size();
    vector<int> dp(n, 1), parent(n, -1);
    int best = 0;
    for (int i = 0; i < n; i++) {
        for (int j = 0; j < i; j++) {
            if (nums[i] % nums[j] == 0 && dp[j] + 1 > dp[i]) {
                dp[i] = dp[j] + 1;
                parent[i] = j;
            }
        }
        if (dp[i] > dp[best]) best = i;
    }
    vector<int> res;
    for (int i = best; i >= 0; i = parent[i]) res.push_back(nums[i]);
    reverse(res.begin(), res.end());
    return res;
}

void print(const vector<int>& v) {
    cout << "[";
    for (size_t i = 0; i < v.size(); i++) {
        if (i) cout << ", ";
        cout << v[i];
    }
    cout << "]" << endl;
}

int main() {
    print(largestDivisibleSubset({3, 5, 10, 20, 21}));
    print(largestDivisibleSubset({1, 3, 6, 24}));
    return 0;
}
