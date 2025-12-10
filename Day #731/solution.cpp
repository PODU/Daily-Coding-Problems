// Day 731: Max profit from a single buy-then-sell.
// Approach: Track running minimum price and best profit in one pass.
// Time: O(n), Space: O(1).
#include <bits/stdc++.h>
using namespace std;

int maxProfit(vector<int>& prices) {
    int minPrice = INT_MAX, best = 0;
    for (int p : prices) {
        minPrice = min(minPrice, p);
        best = max(best, p - minPrice);
    }
    return best;
}

int main() {
    vector<int> prices = {9, 11, 8, 5, 7, 10};
    cout << maxProfit(prices) << "\n"; // 5
    return 0;
}
