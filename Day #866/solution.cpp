// Day 866: Max profit with at most k buy/sell transactions.
// Approach: DP with buy[j]/sell[j] rolling arrays (or greedy when k >= n/2).
// Time: O(n*k), Space: O(k).
#include <bits/stdc++.h>
using namespace std;

int maxProfit(int k, vector<int>& prices) {
    int n = prices.size();
    if (n == 0 || k == 0) return 0;
    if (k >= n / 2) { // unlimited transactions
        int profit = 0;
        for (int i = 1; i < n; i++) if (prices[i] > prices[i-1]) profit += prices[i] - prices[i-1];
        return profit;
    }
    vector<int> buy(k + 1, INT_MIN), sell(k + 1, 0);
    for (int p : prices)
        for (int j = 1; j <= k; j++) {
            buy[j] = max(buy[j], sell[j-1] - p);
            sell[j] = max(sell[j], buy[j] + p);
        }
    return sell[k];
}

int main() {
    vector<int> prices = {5, 2, 4, 0, 1};
    cout << maxProfit(2, prices) << endl; // 3
    return 0;
}
