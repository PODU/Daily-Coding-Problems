// Count connected components via Union-Find with path compression. Time ~O(N+E*alpha), Space O(N).
#include <bits/stdc++.h>
using namespace std;

struct DSU {
    vector<int> p;
    DSU(int n) : p(n) { iota(p.begin(), p.end(), 0); }
    int find(int x) { return p[x] == x ? x : p[x] = find(p[x]); }
    void unite(int a, int b) { p[find(a)] = find(b); }
};

int countGroups(map<int, vector<int>>& adj) {
    int n = adj.size();
    DSU dsu(n);
    for (auto& [u, nbrs] : adj)
        for (int v : nbrs) dsu.unite(u, v);
    set<int> roots;
    for (auto& [u, _] : adj) roots.insert(dsu.find(u));
    return roots.size();
}

int main() {
    map<int, vector<int>> adj = {
        {0, {1, 2}}, {1, {0, 5}}, {2, {0}}, {3, {6}}, {4, {}}, {5, {1}}, {6, {3}}};
    cout << countGroups(adj) << "\n";
}
