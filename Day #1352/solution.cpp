// Arbitrage detection: edge weight = -log(rate); negative cycle => arbitrage. Bellman-Ford.
// Time: O(V*E) = O(V^3), Space: O(V).
#include <bits/stdc++.h>
using namespace std;

bool hasArbitrage(const vector<vector<double>>& rates) {
    int n = rates.size();
    vector<double> dist(n, 0.0); // start: 0 to detect any reachable negative cycle
    // Relax V-1 times
    for (int k = 0; k < n - 1; k++)
        for (int u = 0; u < n; u++)
            for (int v = 0; v < n; v++) {
                double w = -log(rates[u][v]);
                if (dist[u] + w < dist[v] - 1e-12)
                    dist[v] = dist[u] + w;
            }
    // One more pass: if we can still relax, there is a negative cycle
    for (int u = 0; u < n; u++)
        for (int v = 0; v < n; v++) {
            double w = -log(rates[u][v]);
            if (dist[u] + w < dist[v] - 1e-12)
                return true;
        }
    return false;
}

int main() {
    vector<vector<double>> rates = {
        {1.0, 2.0, 1.0},
        {0.5, 1.0, 4.0},
        {1.0, 0.25, 1.0}
    };
    cout << "Arbitrage exists: " << (hasArbitrage(rates) ? "true" : "false") << "\n";
    return 0;
}
