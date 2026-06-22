// Deduce coin denominations from ways[] via incremental unbounded-knapsack DP.
// Time O(N^2), Space O(N).
#include <bits/stdc++.h>
using namespace std;

vector<int> findDenominations(const vector<int>& ways) {
    int n = ways.size();
    vector<long long> dp(n, 0);
    dp[0] = 1;
    vector<int> coins;
    for (int i = 1; i < n; ++i) {
        if (ways[i] != dp[i]) {
            coins.push_back(i);
            for (int j = i; j < n; ++j) dp[j] += dp[j - i];
        }
    }
    return coins;
}

int main() {
    vector<int> ways = {1, 0, 1, 1, 2};
    vector<int> coins = findDenominations(ways);
    cout << "[";
    for (size_t i = 0; i < coins.size(); ++i) {
        if (i) cout << ", ";
        cout << coins[i];
    }
    cout << "]" << endl;
    return 0;
}
