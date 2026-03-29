// Day 1276: Partition a multiset into two subsets of equal sum.
// Subset-sum DP (can we reach totalSum/2?). Time O(n*S), Space O(S).
#include <bits/stdc++.h>
using namespace std;

bool canPartition(const vector<int>& nums) {
    int total = 0;
    for (int x : nums) total += x;
    if (total % 2) return false;
    int target = total / 2;
    vector<char> dp(target + 1, 0);
    dp[0] = 1;
    for (int x : nums)
        for (int s = target; s >= x; --s)
            if (dp[s - x]) dp[s] = 1;
    return dp[target] != 0;
}

int main() {
    cout << (canPartition({15, 5, 20, 10, 35, 15, 10}) ? "true" : "false") << "\n";
    cout << (canPartition({15, 5, 20, 10, 35}) ? "true" : "false") << "\n";
    return 0;
}
