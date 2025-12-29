// Max profit single buy-then-sell: track min price so far and max profit in one pass. O(n) time, O(1) space.
#include <iostream>
#include <vector>
#include <climits>
#include <algorithm>
using namespace std;

int maxProfit(const vector<int>& prices) {
    int minPrice = INT_MAX, best = 0;
    for (int p : prices) {
        minPrice = min(minPrice, p);
        best = max(best, p - minPrice);
    }
    return best;
}

int main() {
    cout << maxProfit({9, 11, 8, 5, 7, 10}) << "\n";
    return 0;
}
