// Day 1111 - Lazy bartender (minimum set cover)
// Approach: exact set cover via DP over bitmask of covered customers.
// Time: O(D * 2^C), Space: O(2^C).
#include <bits/stdc++.h>
using namespace std;

int minDrinks(const map<int, vector<int>>& preferences) {
    vector<int> customers;
    for (auto& kv : preferences) customers.push_back(kv.first);
    int n = customers.size();
    unordered_map<int,int> cidx;
    for (int i = 0; i < n; ++i) cidx[customers[i]] = i;

    unordered_map<int,int> drinkMask;
    for (auto& kv : preferences)
        for (int d : kv.second)
            drinkMask[d] |= (1 << cidx[kv.first]);

    int full = (1 << n) - 1;
    const int INF = INT_MAX;
    vector<int> dp(1 << n, INF);
    dp[0] = 0;
    for (int s = 0; s < (1 << n); ++s) {
        if (dp[s] == INF) continue;
        for (auto& kv : drinkMask) {
            int ns = s | kv.second;
            if (dp[ns] > dp[s] + 1) dp[ns] = dp[s] + 1;
        }
    }
    return dp[full];
}

int main() {
    map<int, vector<int>> preferences = {
        {0, {0, 1, 3, 6}},
        {1, {1, 4, 7}},
        {2, {2, 4, 7, 5}},
        {3, {3, 2, 5}},
        {4, {5, 8}},
    };
    cout << minDrinks(preferences) << endl; // 2
    return 0;
}
