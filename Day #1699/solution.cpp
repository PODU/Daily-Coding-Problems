// Count ways to roll N dice (faces each) summing to total via rolling 1D DP.
// Time O(N*total*faces), Space O(total).
#include <bits/stdc++.h>
using namespace std;

long long throwDice(int n, int faces, int total) {
    vector<long long> dp(total + 1, 0);
    dp[0] = 1;
    for (int k = 0; k < n; ++k) {
        vector<long long> ndp(total + 1, 0);
        for (int s = 0; s <= total; ++s) {
            if (!dp[s]) continue;
            for (int f = 1; f <= faces && s + f <= total; ++f)
                ndp[s + f] += dp[s];
        }
        dp = ndp;
    }
    return dp[total];
}

int main() {
    cout << throwDice(3, 6, 7) << endl;
    return 0;
}
