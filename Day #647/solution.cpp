// Partition equal subset sum: if total odd -> false, else subset-sum DP for sum/2.
// Time O(n*sum), Space O(sum).
#include <bits/stdc++.h>
using namespace std;

bool canPartition(const vector<int>& nums) {
    int total = 0;
    for (int x : nums) total += x;
    if (total % 2 != 0) return false;
    int target = total / 2;
    vector<bool> dp(target + 1, false);
    dp[0] = true;
    for (int x : nums)
        for (int j = target; j >= x; --j)
            if (dp[j - x]) dp[j] = true;
    return dp[target];
}

int main() {
    vector<int> a = {15, 5, 20, 10, 35, 15, 10};
    vector<int> b = {15, 5, 20, 10, 35};
    cout << (canPartition(a) ? "true" : "false") << "\n";
    cout << (canPartition(b) ? "true" : "false") << "\n";
    return 0;
}
