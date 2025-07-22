// Staircase ways with step set X: dp[i] = sum dp[i-x] for x in X.
// Time: O(N*|X|), Space: O(N).
#include <bits/stdc++.h>
using namespace std;

long long staircase(int n, const vector<int>& X) {
    vector<long long> dp(n + 1, 0);
    dp[0] = 1;
    for (int i = 1; i <= n; i++)
        for (int x : X)
            if (i - x >= 0) dp[i] += dp[i - x];
    return dp[n];
}

int main() {
    cout << staircase(4, {1, 2}) << "\n"; // 5
    return 0;
}
