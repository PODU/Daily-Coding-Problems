// Minimum spanning tree cost via Kruskal + union-find over undirected weighted graph.
// Time: O(E log E), Space: O(V + E).
#include <bits/stdc++.h>
using namespace std;

struct DSU {
    vector<int> p;
    DSU(int n): p(n) { iota(p.begin(), p.end(), 0); }
    int find(int x) { return p[x] == x ? x : p[x] = find(p[x]); }
    bool unite(int a, int b) { a = find(a); b = find(b); if (a == b) return false; p[a] = b; return true; }
};

int main() {
    // pipes adjacency: node -> {neighbor: weight}
    map<string, map<string,int>> pipes = {
        {"plant", {{"A",1},{"B",5},{"C",20}}},
        {"A", {{"C",15}}},
        {"B", {{"C",10}}},
        {"C", {}}
    };

    // index nodes
    map<string,int> id;
    for (auto &kv : pipes) if (!id.count(kv.first)) { int n = id.size(); id[kv.first] = n; }
    // collect undirected edges (dedup)
    set<tuple<int,string,string>> seen;
    vector<tuple<int,int,int>> edges; // w, u, v
    for (auto &kv : pipes) {
        for (auto &e : kv.second) {
            string a = kv.first, b = e.first;
            if (a > b) swap(a, b);
            if (seen.insert({e.second, a, b}).second)
                edges.push_back({e.second, id[a], id[b]});
        }
    }
    sort(edges.begin(), edges.end());

    DSU dsu(id.size());
    int total = 0;
    for (auto &t : edges)
        if (dsu.unite(get<1>(t), get<2>(t))) total += get<0>(t);

    cout << total << endl;
    return 0;
}
