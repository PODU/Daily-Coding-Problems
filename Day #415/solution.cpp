// Day 415: Max stock profit, unlimited transactions with a per-transaction fee.
// DP two states: cash (no stock) and hold (holding). Time O(N), Space O(1).
#include <bits/stdc++.h>
using namespace std;

int maxProfit(const vector<int>& prices, int fee) {
    if (prices.empty()) return 0;
    long long cash = 0, hold = -prices[0];
    for (size_t i = 1; i < prices.size(); ++i) {
        cash = max(cash, hold + prices[i] - fee);
        hold = max(hold, cash - prices[i]);
    }
    return (int)cash;
}

int main() {
    cout << maxProfit({1, 3, 2, 8, 4, 10}, 2) << "\n"; // 9
    return 0;
}
