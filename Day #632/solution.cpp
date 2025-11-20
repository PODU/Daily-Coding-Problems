// Day 632: Deduce coin denominations from a "ways to make change" array.
// Approach: reverse coin-change DP. Scan amounts; if ways[i] exceeds the count
// reachable with coins found so far, i is itself a denomination.
// Time: O(N * D), Space: O(N).
#include <bits/stdc++.h>
using namespace std;

vector<int> findDenominations(const vector<long long>& ways) {
    int n = ways.size();
    vector<long long> dp(n, 0);
    dp[0] = 1;
    vector<int> coins;
    for (int i = 1; i < n; i++) {
        if (dp[i] < ways[i]) {          // i is a new denomination
            coins.push_back(i);
            for (int j = i; j < n; j++) dp[j] += dp[j - i];
        }
    }
    return coins;
}

int main() {
    vector<long long> ways = {1, 0, 1, 1, 2};
    auto coins = findDenominations(ways);
    for (size_t i = 0; i < coins.size(); i++) {
        if (i + 1 == coins.size() && coins.size() > 1) cout << "and ";
        cout << coins[i];
        if (i + 1 < coins.size()) cout << ", ";
    }
    cout << "\n";
    return 0;
}
