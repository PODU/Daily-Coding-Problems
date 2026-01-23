// Day 941: Arbitrage exists iff a cycle has product of rates > 1, i.e. a negative cycle
// under weights w = -log(rate). Bellman-Ford. Time O(V^3), Space O(V).
#include <bits/stdc++.h>
using namespace std;

bool hasArbitrage(const vector<vector<double>>& rate) {
    int n = (int)rate.size();
    vector<double> dist(n, 0.0); // virtual source connected to all with weight 0
    // Relax n-1 times.
    for (int iter = 0; iter < n - 1; ++iter)
        for (int u = 0; u < n; ++u)
            for (int v = 0; v < n; ++v) {
                double w = -log(rate[u][v]);
                if (dist[u] + w < dist[v] - 1e-12) dist[v] = dist[u] + w;
            }
    // One more pass: any relaxation => negative cycle => arbitrage.
    for (int u = 0; u < n; ++u)
        for (int v = 0; v < n; ++v) {
            double w = -log(rate[u][v]);
            if (dist[u] + w < dist[v] - 1e-12) return true;
        }
    return false;
}

int main() {
    vector<vector<double>> rate = {
        {1.0, 2.0, 1.0},
        {0.5, 1.0, 2.0},
        {1.0, 0.5, 1.0}};
    cout << (hasArbitrage(rate) ? "true" : "false") << "\n"; // true (0->1->2->0 = 4 > 1)
    return 0;
}
