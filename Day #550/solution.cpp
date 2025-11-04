// Arbitrage detection: weight=-log(rate), Bellman-Ford finds a negative-weight cycle => arbitrage. O(V^3) (V*E edges, V-1 passes).
#include <bits/stdc++.h>
using namespace std;

bool hasArbitrage(const vector<vector<double>>& rates) {
    int n = rates.size();
    vector<double> dist(n, 0.0); // virtual source: all start at 0
    for (int it = 0; it < n - 1; ++it) {
        for (int u = 0; u < n; ++u)
            for (int v = 0; v < n; ++v) {
                double w = -log(rates[u][v]);
                if (dist[u] + w < dist[v] - 1e-12) dist[v] = dist[u] + w;
            }
    }
    for (int u = 0; u < n; ++u)
        for (int v = 0; v < n; ++v) {
            double w = -log(rates[u][v]);
            if (dist[u] + w < dist[v] - 1e-12) return true; // still relaxes => negative cycle
        }
    return false;
}

int main() {
    vector<vector<double>> arb = {{1, 0.5, 0.2}, {2, 1, 0.5}, {5, 2, 1}};
    vector<vector<double>> consistent = {{1, 2, 4}, {0.5, 1, 2}, {0.25, 0.5, 1}};
    cout << (hasArbitrage(arb) ? "true" : "false") << "\n";
    cout << (hasArbitrage(consistent) ? "true" : "false") << "\n";
    return 0;
}
