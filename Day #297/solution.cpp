// Exact set cover via BFS/DP over bitmask of customers; each drink = bitmask of customers accepting it.
// Time: O(2^m * drinks), Space: O(2^m)  (m = number of customers, small).
#include <bits/stdc++.h>
using namespace std;

int main() {
    // preferences[customer] = list of acceptable drinks
    map<int, vector<int>> preferences = {
        {0, {0,1,3,6}}, {1, {1,4,7}}, {2, {2,4,7,5}}, {3, {3,2,5}}, {4, {5,8}}
    };
    int m = preferences.size();
    int full = (1 << m) - 1;

    // For each drink, bitmask of customers who accept it.
    map<int,int> drinkMask;
    for (auto &p : preferences) {
        int cust = p.first;
        for (int d : p.second) drinkMask[d] |= (1 << cust);
    }

    vector<int> dp(full + 1, INT_MAX);
    dp[0] = 0;
    queue<int> q;
    q.push(0);
    while (!q.empty()) {
        int mask = q.front(); q.pop();
        if (mask == full) continue;
        for (auto &dm : drinkMask) {
            int nm = mask | dm.second;
            if (dp[nm] > dp[mask] + 1) {
                dp[nm] = dp[mask] + 1;
                q.push(nm);
            }
        }
    }
    cout << dp[full] << endl;
    return 0;
}
