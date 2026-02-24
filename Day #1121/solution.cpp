// Day 1121 - Max stock profit with a transaction fee, unlimited transactions
// State machine DP: best cash (not holding) and best hold. Time: O(n), Space: O(1).
#include <bits/stdc++.h>
using namespace std;

int maxProfit(const vector<int>& prices, int fee) {
    int cash = 0, hold = -prices[0];
    for (size_t i = 1; i < prices.size(); ++i) {
        cash = max(cash, hold + prices[i] - fee);
        hold = max(hold, cash - prices[i]);
    }
    return cash;
}

int main() {
    cout << maxProfit({1, 3, 2, 8, 4, 10}, 2) << endl; // 9
    return 0;
}
