// Day 130: Max profit with at most k buy/sell transactions.
// DP with hold/cash per transaction. O(n*k) time, O(k) space (greedy when k large).
#include <bits/stdc++.h>
using namespace std;

int maxProfit(int k, vector<int>& p) {
    int n = p.size();
    if (n == 0 || k == 0) return 0;
    if (k >= n / 2) { // unlimited transactions
        int prof = 0;
        for (int i = 1; i < n; i++)
            if (p[i] > p[i - 1]) prof += p[i] - p[i - 1];
        return prof;
    }
    vector<int> buy(k + 1, INT_MIN), sell(k + 1, 0);
    for (int price : p)
        for (int j = 1; j <= k; j++) {
            buy[j] = max(buy[j], sell[j - 1] - price);
            sell[j] = max(sell[j], buy[j] + price);
        }
    return sell[k];
}

int main() {
    vector<int> prices = {5, 2, 4, 0, 1};
    cout << maxProfit(2, prices) << endl; // 3
    return 0;
}
