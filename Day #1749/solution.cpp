// Day 1749: Max profit with at most k transactions.
// DP with buy/sell states; if k>=n/2 it's unlimited (sum positive diffs).
// Time O(n*k) (or O(n) when unlimited), Space O(k).
#include <bits/stdc++.h>
using namespace std;

int maxProfit(int k, const vector<int>& prices) {
    int n = prices.size();
    if (n < 2 || k == 0) return 0;
    if (k >= n / 2) {
        int profit = 0;
        for (int i = 1; i < n; ++i)
            if (prices[i] > prices[i - 1]) profit += prices[i] - prices[i - 1];
        return profit;
    }
    vector<int> buy(k + 1, INT_MIN), sell(k + 1, 0);
    for (int price : prices) {
        for (int j = 1; j <= k; ++j) {
            buy[j] = max(buy[j], sell[j - 1] - price);
            sell[j] = max(sell[j], buy[j] + price);
        }
    }
    return sell[k];
}

int main() {
    int k = 2;
    vector<int> prices = {5, 2, 4, 0, 1};
    cout << maxProfit(k, prices) << "\n"; // 3
    return 0;
}
