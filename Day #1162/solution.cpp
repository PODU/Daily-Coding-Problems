// Root tree, DFS subtree sizes; count non-root nodes with even subtree size (cuttable parent edges). O(n) time, O(n) space.
#include <bits/stdc++.h>
using namespace std;

vector<vector<int>> adj;
int answer = 0;

int dfs(int u, int parent) {
    int size = 1;
    for (int v : adj[u]) {
        if (v != parent) size += dfs(v, u);
    }
    if (parent != -1 && size % 2 == 0) answer++;
    return size;
}

int main() {
    int n = 8;
    adj.assign(n + 1, {});
    auto addEdge = [&](int a, int b) { adj[a].push_back(b); adj[b].push_back(a); };
    addEdge(1, 2);
    addEdge(1, 3);
    addEdge(3, 4);
    addEdge(3, 5);
    addEdge(4, 6);
    addEdge(4, 7);
    addEdge(4, 8);
    dfs(1, -1);
    cout << answer << "\n";
    return 0;
}
