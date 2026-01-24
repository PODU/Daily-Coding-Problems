// Day 945: Longest path (diameter) in a weighted tree.
// DFS: at each node combine its two deepest downward branches. Time O(V+E), Space O(V).
#include <bits/stdc++.h>
using namespace std;

map<string, vector<pair<string,int>>> adj;
int best = 0;

// Returns the longest downward path length from node (excluding edge above it).
int dfs(const string& node, const string& parent) {
    int max1 = 0, max2 = 0; // two largest downward branch lengths
    for (size_t i = 0; i < adj[node].size(); ++i) {
        const string& nb = adj[node][i].first;
        int w = adj[node][i].second;
        if (nb == parent) continue;
        int d = dfs(nb, node) + w;
        if (d > max1) { max2 = max1; max1 = d; }
        else if (d > max2) { max2 = d; }
    }
    best = max(best, max1 + max2); // path passing through this node
    return max1;
}

void addEdge(const string& u, const string& v, int w) {
    adj[u].push_back({v, w});
    adj[v].push_back({u, w});
}

int main() {
    addEdge("a","b",3); addEdge("a","c",5); addEdge("a","d",8);
    addEdge("d","e",2); addEdge("d","f",4);
    addEdge("e","g",1); addEdge("e","h",1);
    dfs("a", "");
    cout << best << "\n"; // 17 (path c -> a -> d -> f)
    return 0;
}
