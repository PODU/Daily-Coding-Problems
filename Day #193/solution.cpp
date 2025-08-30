// Day 193: Max profit, unlimited transactions, fee charged per completed transaction.
// State DP: cash (no stock) / hold (holding). Time O(n), Space O(1).
#include <bits/stdc++.h>
using namespace std;

long long maxProfit(const vector<int>& prices, int fee) {
    if (prices.empty()) return 0;
    long long cash = 0, hold = -prices[0];
    for (size_t i = 1; i < prices.size(); i++) {
        cash = max(cash, hold + prices[i] - fee);
        hold = max(hold, cash - prices[i]);
    }
    return cash;
}

int main() {
    cout << maxProfit({1, 3, 2, 8, 4, 10}, 2) << "\n";
    return 0;
}
