// Arbitrage = negative cycle in graph with weights -log(rate). Bellman-Ford.
// O(V^3) for a dense rate table.
#include <bits/stdc++.h>
using namespace std;

bool hasArbitrage(vector<vector<double>>& rate){
    int n = rate.size();
    vector<vector<double>> w(n, vector<double>(n));
    for(int i = 0; i < n; i++)
        for(int j = 0; j < n; j++)
            w[i][j] = -log(rate[i][j]);
    vector<double> dist(n, 0.0); // virtual source reaching all with 0
    for(int it = 0; it < n - 1; it++)
        for(int u = 0; u < n; u++)
            for(int v = 0; v < n; v++)
                if(dist[u] + w[u][v] < dist[v]) dist[v] = dist[u] + w[u][v];
    for(int u = 0; u < n; u++)
        for(int v = 0; v < n; v++)
            if(dist[u] + w[u][v] < dist[v] - 1e-12) return true;
    return false;
}

int main(){
    // 1 unit of i -> rate[i][j] units of j.  0.8 * 1.3 = 1.04 > 1 => arbitrage
    vector<vector<double>> rate = {
        {1.0, 0.8, 0.5},
        {1.3, 1.0, 1.9},
        {1.9, 0.5, 1.0}
    };
    cout << (hasArbitrage(rate) ? "true" : "false") << "\n"; // true
    return 0;
}
