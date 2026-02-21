// Day 1107: Max edges removable so every component has an even node count.
// DFS subtree sizes; every non-root node with an even-sized subtree => one cuttable edge.
// Time: O(V+E). Space: O(V).
#include <bits/stdc++.h>
using namespace std;

int answer = 0;

int dfs(int u, int parent, vector<vector<int>>& adj){
    int size = 1;
    for (int v : adj[u]) if (v != parent) size += dfs(v, u, adj);
    if (parent != -1 && size % 2 == 0) answer++;  // cut edge above u
    return size;
}

int main(){
    int n = 8;
    vector<vector<int>> adj(n+1);
    int edges[][2] = {{1,2},{1,3},{3,4},{3,5},{4,6},{4,7},{4,8}};
    for (auto& e : edges){ adj[e[0]].push_back(e[1]); adj[e[1]].push_back(e[0]); }
    answer = 0;
    dfs(1, -1, adj);
    cout << answer << "\n"; // 2  (e.g. cut (1,3) and (3,4) -> {1,2},{3,5},{4,6,7,8})
    return 0;
}
