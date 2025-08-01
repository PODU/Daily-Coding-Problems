// Day 60: Equal-sum partition = subset-sum to total/2, via bitset DP.
// Time: O(n * sum / 64), Space: O(sum / 64).
#include <bits/stdc++.h>
using namespace std;

bool canPartition(const vector<int>& nums) {
    int total = accumulate(nums.begin(), nums.end(), 0);
    if (total % 2) return false;
    int target = total / 2;
    bitset<100001> dp;
    dp[0] = 1;
    for (int x : nums) dp |= dp << x;
    return dp[target];
}

int main() {
    cout << boolalpha << canPartition({15, 5, 20, 10, 35, 15, 10}) << endl; // true
    cout << boolalpha << canPartition({15, 5, 20, 10, 35}) << endl;          // false
    return 0;
}
