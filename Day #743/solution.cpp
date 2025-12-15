// Find all bridges with Tarjan's DFS: edge (u,v) is a bridge if low[v] > disc[u].
// Time: O(V + E), Space: O(V + E).
#include <bits/stdc++.h>
using namespace std;

vector<vector<int>> adj;
vector<int> disc, low;
vector<pair<int,int>> bridges;
int timer = 0;

void dfs(int u, int parent){
    disc[u] = low[u] = ++timer;
    bool skippedParent = false;
    for(int v : adj[u]){
        if(v == parent && !skippedParent){ skippedParent = true; continue; }
        if(disc[v] == 0){
            dfs(v, u);
            low[u] = min(low[u], low[v]);
            if(low[v] > disc[u]) bridges.push_back({min(u,v), max(u,v)});
        } else {
            low[u] = min(low[u], disc[v]);
        }
    }
}

int main(){
    int n = 5;
    adj.assign(n, {});
    vector<pair<int,int>> edges = {{0,1},{1,2},{2,0},{1,3},{3,4}};
    for(auto& e : edges){ adj[e.first].push_back(e.second); adj[e.second].push_back(e.first); }
    disc.assign(n,0); low.assign(n,0);
    for(int i=0;i<n;i++) if(disc[i]==0) dfs(i,-1);
    sort(bridges.begin(), bridges.end());
    for(auto& b : bridges) cout << "(" << b.first << ", " << b.second << ")\n";
    // (1, 3)
    // (3, 4)
    return 0;
}
