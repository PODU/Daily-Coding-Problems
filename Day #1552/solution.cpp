// Max profit single buy-then-sell: track running min price and best (price - min). Time O(n), Space O(1).
#include <iostream>
#include <vector>
#include <algorithm>

int maxProfit(const std::vector<int>& prices) {
    if (prices.empty()) return 0;
    int minPrice = prices[0], best = 0;
    for (int p : prices) {
        minPrice = std::min(minPrice, p);
        best = std::max(best, p - minPrice);
    }
    return best;
}

int main() {
    std::vector<int> prices = {9, 11, 8, 5, 7, 10};
    std::cout << maxProfit(prices) << "\n";
    return 0;
}
