// Day 717: Min cost to paint N houses, K colors, no two adjacent same color.
// DP over rows tracking best & second-best previous costs. Time O(N*K), space O(1).
#include <bits/stdc++.h>
using namespace std;

int minCost(const vector<vector<int>>& costs) {
    if (costs.empty()) return 0;
    int K = costs[0].size();
    vector<int> prev = costs[0];
    for (size_t i = 1; i < costs.size(); ++i) {
        // find min and second min of prev
        int m1 = INT_MAX, m2 = INT_MAX, idx = -1;
        for (int k = 0; k < K; ++k) {
            if (prev[k] < m1) { m2 = m1; m1 = prev[k]; idx = k; }
            else if (prev[k] < m2) m2 = prev[k];
        }
        vector<int> cur(K);
        for (int k = 0; k < K; ++k)
            cur[k] = costs[i][k] + (k == idx ? m2 : m1);
        prev = cur;
    }
    return *min_element(prev.begin(), prev.end());
}

int main() {
    vector<vector<int>> costs = {{1, 5, 3}, {2, 9, 4}};
    cout << minCost(costs) << endl; // 5
    return 0;
}
