// Day 47: Max profit from one buy-then-sell. Track min price so far.
// Time: O(n), Space: O(1).
#include <bits/stdc++.h>
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
    cout << maxProfit({9, 11, 8, 5, 7, 10}) << endl;
    return 0;
}
