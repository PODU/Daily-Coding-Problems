// Paint House: DP tracking min cost per color using min1/min2 trick.
// Time O(N*K), Space O(1) extra.
#include <bits/stdc++.h>
using namespace std;

int minCost(const vector<vector<int>>& cost) {
    if (cost.empty()) return 0;
    int K = cost[0].size();
    vector<int> prev = cost[0];
    for (size_t i = 1; i < cost.size(); ++i) {
        // find min1, min2 indices of prev
        int min1 = -1, min2 = -1;
        for (int k = 0; k < K; ++k) {
            if (min1 == -1 || prev[k] < prev[min1]) { min2 = min1; min1 = k; }
            else if (min2 == -1 || prev[k] < prev[min2]) { min2 = k; }
        }
        vector<int> cur(K);
        for (int k = 0; k < K; ++k) {
            int best = (k == min1) ? prev[min2] : prev[min1];
            cur[k] = cost[i][k] + best;
        }
        prev = cur;
    }
    return *min_element(prev.begin(), prev.end());
}

int main() {
    vector<vector<int>> cost = {{1,5,3},{2,9,4}};
    cout << "Minimum cost = " << minCost(cost) << "\n";
    return 0;
}
