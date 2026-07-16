// Bipartite check via BFS 2-coloring. O(V+E) time, O(V) space.
#include <bits/stdc++.h>
using namespace std;

bool isBipartite(int n, vector<vector<int>>& adj){
    vector<int> color(n, -1);
    for(int s = 0; s < n; s++){
        if(color[s] != -1) continue;
        queue<int> q; q.push(s); color[s] = 0;
        while(!q.empty()){
            int u = q.front(); q.pop();
            for(int v : adj[u]){
                if(color[v] == -1){ color[v] = color[u] ^ 1; q.push(v); }
                else if(color[v] == color[u]) return false;
            }
        }
    }
    return true;
}

int main(){
    int n = 4;
    vector<vector<int>> adj(n);
    auto add = [&](int a,int b){ adj[a].push_back(b); adj[b].push_back(a); };
    add(0,1); add(1,2); add(2,3); add(3,0); // even cycle -> bipartite
    cout << (isBipartite(n, adj) ? "true" : "false") << "\n";
    return 0;
}
