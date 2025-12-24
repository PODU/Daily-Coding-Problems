// throw_dice: DP over dice, dp[s] = ways to reach sum s. Rolling array.
// Time O(N*total*faces), Space O(total).
#include <bits/stdc++.h>
using namespace std;

long long throwDice(int N, int faces, int total) {
    vector<long long> dp(total + 1, 0);
    dp[0] = 1;
    for (int i = 0; i < N; i++) {
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
    cout << throwDice(3, 6, 7) << endl;
    return 0;
}
