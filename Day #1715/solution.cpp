// DP states cash/hold; fee charged once per buy-sell on sell. Time O(n), Space O(1).
#include <bits/stdc++.h>
using namespace std;

int maxProfit(const vector<int>& prices, int fee) {
    if (prices.empty()) return 0;
    long cash = 0, hold = -prices[0];
    for (int i = 1; i < (int)prices.size(); i++) {
        cash = max(cash, hold + prices[i] - fee);
        hold = max(hold, cash - prices[i]);
    }
    return (int)cash;
}

int main() {
    vector<int> prices = {1, 3, 2, 8, 4, 10};
    int fee = 2;
    cout << maxProfit(prices, fee) << "\n";
    return 0;
}
