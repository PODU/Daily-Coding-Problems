// Day 862: Find all bridges in a connected undirected graph.
// Approach: Tarjan's DFS using discovery times and low-link values.
// Time: O(V + E), Space: O(V + E).
#include <bits/stdc++.h>
using namespace std;

int timer = 0;
vector<vector<int>> adj;
vector<int> disc, low;
vector<pair<int,int>> bridges;

void dfs(int u, int parent) {
    disc[u] = low[u] = ++timer;
    for (int v : adj[u]) {
        if (v == parent) continue;
        if (disc[v]) low[u] = min(low[u], disc[v]);
        else {
            dfs(v, u);
            low[u] = min(low[u], low[v]);
            if (low[v] > disc[u]) bridges.push_back({min(u,v), max(u,v)});
        }
    }
}

int main() {
    int n = 5;
    adj.assign(n, {});
    auto add = [&](int a, int b){ adj[a].push_back(b); adj[b].push_back(a); };
    add(0,1); add(1,2); add(2,0); add(1,3); add(3,4);
    disc.assign(n, 0); low.assign(n, 0);
    for (int i = 0; i < n; i++) if (!disc[i]) dfs(i, -1);

    sort(bridges.begin(), bridges.end());
    cout << "Bridges:";
    for (auto& b : bridges) cout << " (" << b.first << ", " << b.second << ")";
    cout << endl;   // (1, 3) (3, 4)
    return 0;
}
