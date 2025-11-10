// throw_dice: DP over dice count, dp[s] = ways to reach sum s. Time O(N*total*faces), Space O(total).
#include <bits/stdc++.h>
using namespace std;

long long throw_dice(int N, int faces, int total) {
    vector<long long> dp(total + 1, 0);
    dp[0] = 1;
    for (int d = 1; d <= N; d++) {
        vector<long long> ndp(total + 1, 0);
        for (int s = 0; s <= total; s++) {
            if (dp[s] == 0) continue;
            for (int f = 1; f <= faces && s + f <= total; f++)
                ndp[s + f] += dp[s];
        }
        dp = ndp;
    }
    return dp[total];
}

int main() {
    cout << throw_dice(3, 6, 7) << "\n";
    return 0;
}
