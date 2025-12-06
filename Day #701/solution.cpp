// Day 701: Minimum drinks to satisfy every customer (minimum set cover).
// Approach: each drink -> bitmask of customers it satisfies; DP over customer
// masks for fewest drinks covering all. Time O(2^C * D), Space O(2^C).
#include <bits/stdc++.h>
using namespace std;

int minDrinks(map<int, vector<int>>& prefs) {
    int C = prefs.size();
    vector<int> custIdx; // map customer id -> index 0..C-1
    map<int,int> idx;
    for (auto& [cust, _] : prefs) { idx[cust] = custIdx.size(); custIdx.push_back(cust); }
    map<int,int> drinkMask; // drink -> customers-mask
    for (auto& [cust, list] : prefs)
        for (int d : list) drinkMask[d] |= (1 << idx[cust]);
    int full = (1 << C) - 1;
    vector<int> dp(full + 1, INT_MAX);
    dp[0] = 0;
    for (int mask = 0; mask <= full; ++mask) {
        if (dp[mask] == INT_MAX) continue;
        for (auto& [d, dm] : drinkMask) {
            int nm = mask | dm;
            if (dp[mask] + 1 < dp[nm]) dp[nm] = dp[mask] + 1;
        }
    }
    return dp[full];
}

int main() {
    map<int, vector<int>> prefs = {
        {0, {0,1,3,6}}, {1, {1,4,7}}, {2, {2,4,7,5}}, {3, {3,2,5}}, {4, {5,8}}};
    cout << minDrinks(prefs) << "\n"; // 2
    return 0;
}
