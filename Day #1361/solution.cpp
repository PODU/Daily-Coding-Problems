// Max profit, unlimited transactions with a fixed fee per completed sale.
// DP states cash/hold tracked greedily. Time O(n), Space O(1).
#include <bits/stdc++.h>
using namespace std;

int maxProfit(const vector<int>& prices, int fee) {
    long long cash = 0, hold = LLONG_MIN / 4;
    for (int p : prices) {
        hold = max(hold, cash - p);
        cash = max(cash, hold + p - fee);
    }
    return (int)cash;
}

int main() {
    vector<int> prices = {1, 3, 2, 8, 4, 10};
    int fee = 2;
    cout << maxProfit(prices, fee) << "\n";
    return 0;
}
