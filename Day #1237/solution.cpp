// Count connected components (friend groups) via Union-Find.
// Time O(V+E a(V)), Space O(V).
#include <bits/stdc++.h>
using namespace std;

struct DSU {
    vector<int> p;
    DSU(int n) : p(n) { iota(p.begin(), p.end(), 0); }
    int find(int x) { while (p[x] != x) { p[x] = p[p[x]]; x = p[x]; } return x; }
    void unite(int a, int b) { p[find(a)] = find(b); }
};

int countGroups(const map<int, vector<int>>& adj) {
    int n = adj.size();
    DSU dsu(n);
    for (auto& kv : adj)
        for (int v : kv.second)
            dsu.unite(kv.first, v);
    set<int> roots;
    for (auto& kv : adj) roots.insert(dsu.find(kv.first));
    return roots.size();
}

int main() {
    map<int, vector<int>> adj = {{0, {1, 2}}, {1, {0, 5}}, {2, {0}},
                                 {3, {6}}, {4, {}}, {5, {1}}, {6, {3}}};
    cout << countGroups(adj) << "\n";
    return 0;
}
