// Day 160: Weighted tree diameter. One DFS; each node returns its longest
// downward branch, while we combine the top two branches. Time O(V+E).
#include <bits/stdc++.h>
using namespace std;

map<string, vector<pair<string,int>>> adj;
long long best = 0;

// returns longest downward path length from node
long long dfs(const string& node, const string& parent) {
    long long top1 = 0, top2 = 0;
    for (auto& edge : adj[node]) {
        const string& nb = edge.first;
        int w = edge.second;
        if (nb == parent) continue;
        long long d = w + dfs(nb, node);
        if (d > top1) { top2 = top1; top1 = d; }
        else if (d > top2) { top2 = d; }
    }
    best = max(best, top1 + top2);
    return top1;
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
    cout << best << "\n"; // 17
    return 0;
}
