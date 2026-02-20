// Day 1105: Partition multiset into two equal-sum subsets (subset-sum DP).
// If total odd -> false; else can we reach total/2. Time: O(N*sum). Space: O(sum).
#include <bits/stdc++.h>
using namespace std;

bool canPartition(const vector<int>& nums){
    int total = accumulate(nums.begin(), nums.end(), 0);
    if (total & 1) return false;
    int target = total / 2;
    vector<char> dp(target+1, 0);
    dp[0] = 1;
    for (int x : nums)
        for (int s = target; s >= x; s--)
            if (dp[s-x]) dp[s] = 1;
    return dp[target];
}

int main(){
    cout << (canPartition({15,5,20,10,35,15,10}) ? "true" : "false") << "\n"; // true
    cout << (canPartition({15,5,20,10,35}) ? "true" : "false") << "\n";        // false
    return 0;
}
