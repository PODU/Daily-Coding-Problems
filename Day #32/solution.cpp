// Currency arbitrage: weights = -log(rate), Bellman-Ford detects negative cycle. O(V*E).
#include <bits/stdc++.h>
using namespace std;

bool hasArbitrage(const vector<vector<double>>& rates) {
    int n = rates.size();
    vector<vector<double>> w(n, vector<double>(n));
    for (int i = 0; i < n; i++)
        for (int j = 0; j < n; j++)
            w[i][j] = -log(rates[i][j]);
    vector<double> dist(n, 0.0); // virtual super-source reaching all nodes at 0
    for (int it = 0; it < n - 1; it++)
        for (int u = 0; u < n; u++)
            for (int v = 0; v < n; v++)
                if (dist[u] + w[u][v] < dist[v] - 1e-12) dist[v] = dist[u] + w[u][v];
    for (int u = 0; u < n; u++)
        for (int v = 0; v < n; v++)
            if (dist[u] + w[u][v] < dist[v] - 1e-12) return true;
    return false;
}

int main() {
    vector<vector<double>> r1 = {{1.0, 0.7, 0.5}, {1.4, 1.0, 0.7}, {2.1, 1.4, 1.0}};
    vector<vector<double>> r2 = {{1.0, 2.0, 4.0}, {0.5, 1.0, 2.0}, {0.25, 0.5, 1.0}};
    cout << (hasArbitrage(r1) ? "true" : "false") << "\n";
    cout << (hasArbitrage(r2) ? "true" : "false") << "\n";
    return 0;
}
