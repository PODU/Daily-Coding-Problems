// Min coins via DP over amounts (optimal for arbitrary denominations).
// Greedy also works for canonical US coins {1,5,10,25}. Time: O(n*k), Space: O(n).
#include <bits/stdc++.h>
using namespace std;

int minCoins(int n, const vector<int>& coins) {
    const int INF = INT_MAX;
    vector<int> dp(n + 1, INF);
    dp[0] = 0;
    for (int a = 1; a <= n; ++a) {
        for (int c : coins) {
            if (c <= a && dp[a - c] != INF) {
                dp[a] = min(dp[a], dp[a - c] + 1);
            }
        }
    }
    return dp[n];
}

int main() {
    vector<int> coins = {1, 5, 10, 25};
    int n = 16;
    cout << minCoins(n, coins) << "\n";
    return 0;
}
