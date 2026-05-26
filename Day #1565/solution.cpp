// Subset-sum DP: partition into equal halves iff target=sum/2 reachable. Time O(n*sum), Space O(sum).
#include <bits/stdc++.h>
using namespace std;

bool canPartition(const vector<int>& nums) {
    int total = accumulate(nums.begin(), nums.end(), 0);
    if (total % 2) return false;
    int target = total / 2;
    vector<char> dp(target + 1, false);
    dp[0] = true;
    for (int x : nums)
        for (int s = target; s >= x; --s)
            if (dp[s - x]) dp[s] = true;
    return dp[target];
}

int main() {
    vector<int> nums = {15, 5, 20, 10, 35, 15, 10};
    cout << (canPartition(nums) ? "true" : "false") << endl;
    return 0;
}
