// Longest weighted path (diameter) in a tree via two DFS passes.
// DFS from any node finds one endpoint; DFS from it finds the other + path. Time: O(V+E). Space: O(V).
#include <bits/stdc++.h>
using namespace std;

map<string, vector<pair<string,int>>> g;

pair<string,long long> farthest(const string& src, map<string,string>& parent) {
    parent.clear();
    map<string,bool> vis;
    string bestNode = src; long long bestDist = 0;
    function<void(string,long long)> dfs = [&](string u, long long d) {
        vis[u] = true;
        if (d > bestDist) { bestDist = d; bestNode = u; }
        for (auto& e : g[u]) if (!vis[e.first]) { parent[e.first] = u; dfs(e.first, d + e.second); }
    };
    dfs(src, 0);
    return {bestNode, bestDist};
}

int main() {
    auto add = [&](string a, string b, int w){ g[a].push_back({b,w}); g[b].push_back({a,w}); };
    add("a","b",3); add("a","c",5); add("a","d",8);
    add("d","e",2); add("d","f",4); add("e","g",1); add("e","h",1);

    map<string,string> dummy;
    auto u = farthest("a", dummy).first;          // one endpoint
    map<string,string> parent;
    auto res = farthest(u, parent);               // other endpoint + parents
    string v = res.first; long long len = res.second;

    vector<string> path;
    for (string cur = v; ; cur = parent[cur]) { path.push_back(cur); if (cur == u) break; }
    // path is built from v back to u (the first-found endpoint), reading as the full diameter

    for (size_t i = 0; i < path.size(); i++) { if (i) cout << " -> "; cout << path[i]; }
    cout << ", with a length of " << len << "\n";
    // c -> a -> d -> f, with a length of 17
    return 0;
}
