// Day 408: Max profit with at most k stock transactions.
// Approach: DP tracking best buy/sell state per transaction in one pass.
// Time: O(n*k), Space: O(k). Example k=2, [5,2,4,0,1] -> 3.
#include <bits/stdc++.h>
using namespace std;

int maxProfit(int k, const vector<int>& prices) {
    int n = prices.size();
    if (n == 0 || k == 0) return 0;
    // If k >= n/2, unlimited transactions are possible.
    if (k >= n / 2) {
        int profit = 0;
        for (int i = 1; i < n; i++)
            if (prices[i] > prices[i - 1]) profit += prices[i] - prices[i - 1];
        return profit;
    }
    vector<int> buy(k + 1, INT_MIN), sell(k + 1, 0);
    for (int price : prices) {
        for (int t = 1; t <= k; t++) {
            buy[t] = max(buy[t], sell[t - 1] - price);
            sell[t] = max(sell[t], buy[t] + price);
        }
    }
    return sell[k];
}

int main() {
    cout << maxProfit(2, {5, 2, 4, 0, 1}) << endl; // 3
    return 0;
}
