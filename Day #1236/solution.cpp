// Count ways to roll N dice (faces each) summing to total via DP.
// Time O(N*faces*total), Space O(total).
#include <bits/stdc++.h>
using namespace std;

long long throwDice(int N, int faces, int total) {
    vector<long long> dp(total + 1, 0);
    dp[0] = 1;
    for (int d = 0; d < N; ++d) {
        vector<long long> nxt(total + 1, 0);
        for (int s = 0; s <= total; ++s)
            if (dp[s])
                for (int f = 1; f <= faces && s + f <= total; ++f)
                    nxt[s + f] += dp[s];
        dp = nxt;
    }
    return dp[total];
}

int main() {
    cout << throwDice(3, 6, 7) << "\n";
    return 0;
}
