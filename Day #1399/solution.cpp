// DP over rows, tracking the two smallest running totals so each house picks
// the best previous color != its own. Time O(N*K), Space O(1) extra.
#include <bits/stdc++.h>
using namespace std;

int minCost(const vector<vector<int>>& costs) {
    if (costs.empty()) return 0;
    int K = costs[0].size();
    vector<int> prev = costs[0];
    for (size_t i = 1; i < costs.size(); i++) {
        // find smallest and second smallest of prev with their index
        int m1 = INT_MAX, m2 = INT_MAX, idx1 = -1;
        for (int k = 0; k < K; k++) {
            if (prev[k] < m1) { m2 = m1; m1 = prev[k]; idx1 = k; }
            else if (prev[k] < m2) m2 = prev[k];
        }
        vector<int> cur(K);
        for (int k = 0; k < K; k++)
            cur[k] = costs[i][k] + (k == idx1 ? m2 : m1);
        prev = cur;
    }
    return *min_element(prev.begin(), prev.end());
}

int main() {
    vector<vector<int>> costs = {{1, 5, 3}, {2, 9, 4}};
    cout << minCost(costs) << "\n"; // 5
    return 0;
}
