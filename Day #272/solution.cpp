// Day 272: throw_dice(N, faces, total) -> number of ways to reach total.
// 1-D DP over rolling sums. Time O(N*total*faces), Space O(total).
#include <bits/stdc++.h>
using namespace std;

long long throwDice(int N, int faces, int total) {
    vector<long long> dp(total + 1, 0);
    dp[0] = 1;
    for (int d = 0; d < N; d++) {
        vector<long long> ndp(total + 1, 0);
        for (int t = 0; t <= total; t++) {
            if (!dp[t]) continue;
            for (int f = 1; f <= faces && t + f <= total; f++)
                ndp[t + f] += dp[t];
        }
        dp = ndp;
    }
    return dp[total];
}

int main() {
    cout << throwDice(3, 6, 7) << "\n"; // 15
    return 0;
}
