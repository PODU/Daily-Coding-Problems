// Day 1476: Max profit from a single buy-then-sell.
// Track the minimum price so far and the best profit in one pass.
// Time O(N), Space O(1).
#include <bits/stdc++.h>
using namespace std;

int maxProfit(const vector<int>& prices) {
    int minPrice = INT_MAX, best = 0;
    for (int p : prices) {
        if (p < minPrice) minPrice = p;
        else if (p - minPrice > best) best = p - minPrice;
    }
    return best;
}

int main() {
    cout << maxProfit({9, 11, 8, 5, 7, 10}) << "\n";  // 5
}
