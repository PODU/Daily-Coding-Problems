// Remove max edges so each resulting subtree has an even node count.
// DFS subtree sizes; count non-root nodes whose subtree size is even (each = one removable edge above it). O(n) time/space.
#include <iostream>
#include <vector>
using namespace std;

vector<vector<int>> adj;
int removable = 0;

int dfs(int u, int parent) {
    int size = 1;
    for (int v : adj[u]) if (v != parent) size += dfs(v, u);
    if (parent != -1 && size % 2 == 0) removable++;
    return size;
}

int main() {
    int n = 8;
    adj.assign(n + 1, {});
    vector<pair<int, int>> edges = {{1,2},{1,3},{3,4},{3,5},{4,6},{4,7},{4,8}};
    for (auto& e : edges) {
        adj[e.first].push_back(e.second);
        adj[e.second].push_back(e.first);
    }
    dfs(1, -1);
    cout << removable << "\n";
    return 0;
}
