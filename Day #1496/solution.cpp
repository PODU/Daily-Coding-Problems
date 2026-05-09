// Day 1496: Min cost to paint N houses with K colors, no two adjacent same color.
// Approach: DP tracking previous row's min & 2nd-min (+min index). Time O(N*K), Space O(1).
#include <bits/stdc++.h>
using namespace std;

int minCost(const vector<vector<int>>& costs) {
    if (costs.empty()) return 0;
    long long prevMin1 = 0, prevMin2 = 0; int prevIdx = -1;
    for (const auto& row : costs) {
        long long curMin1 = LLONG_MAX, curMin2 = LLONG_MAX; int curIdx = -1;
        for (int k = 0; k < (int)row.size(); ++k) {
            long long add = (k == prevIdx) ? prevMin2 : prevMin1;
            long long c = row[k] + add;
            if (c < curMin1) { curMin2 = curMin1; curMin1 = c; curIdx = k; }
            else if (c < curMin2) { curMin2 = c; }
        }
        prevMin1 = curMin1; prevMin2 = curMin2; prevIdx = curIdx;
    }
    return (int)prevMin1;
}

int main() {
    vector<vector<int>> costs = {{1, 5, 3}, {2, 9, 4}};
    cout << minCost(costs) << "\n"; // expected 5
    return 0;
}
