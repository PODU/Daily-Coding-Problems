// Day 1279: Lazy bartender = minimum set cover over customers.
// DP over customer bitmask. Time O(2^C * D), Space O(2^C). C=#customers, D=#drinks.
#include <bits/stdc++.h>
using namespace std;

int minDrinks(const vector<vector<int>>& prefs) {
    int C = prefs.size();
    unordered_map<int, int> drinkMask; // drink -> bitmask of customers who accept it
    for (int cust = 0; cust < C; ++cust)
        for (int d : prefs[cust]) drinkMask[d] |= (1 << cust);
    int full = (1 << C) - 1;
    vector<int> dp(1 << C, INT_MAX);
    dp[0] = 0;
    for (int mask = 0; mask <= full; ++mask) {
        if (dp[mask] == INT_MAX) continue;
        for (auto& kv : drinkMask) {
            int nm = mask | kv.second;
            if (dp[mask] + 1 < dp[nm]) dp[nm] = dp[mask] + 1;
        }
    }
    return dp[full];
}

int main() {
    vector<vector<int>> prefs = {
        {0, 1, 3, 6}, {1, 4, 7}, {2, 4, 7, 5}, {3, 2, 5}, {5, 8}};
    cout << minDrinks(prefs) << "\n"; // 2
    return 0;
}
