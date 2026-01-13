// Paint houses: DP tracking two smallest costs of previous row -> O(N*K) time, O(1) extra space.
// For each house we only need the min and second-min of the previous row to avoid same color.
#include <bits/stdc++.h>
using namespace std;

int minCost(const vector<vector<int>>& costs) {
    if (costs.empty()) return 0;
    int prevMin = 0, prevSecond = 0, prevIdx = -1;
    for (const auto& row : costs) {
        int curMin = INT_MAX, curSecond = INT_MAX, curIdx = -1;
        for (int c = 0; c < (int)row.size(); ++c) {
            int cost = row[c] + (c == prevIdx ? prevSecond : prevMin);
            if (cost < curMin) {
                curSecond = curMin;
                curMin = cost;
                curIdx = c;
            } else if (cost < curSecond) {
                curSecond = cost;
            }
        }
        prevMin = curMin;
        prevSecond = curSecond;
        prevIdx = curIdx;
    }
    return prevMin;
}

int main() {
    vector<vector<int>> costs = {{1, 5, 3}, {2, 9, 4}};
    cout << minCost(costs) << endl;
    return 0;
}
