// Day 279: Count friend groups = connected components via Union-Find.
// Time O(V + E * alpha(V)), Space O(V).
#include <bits/stdc++.h>
using namespace std;

struct DSU {
    vector<int> p, r;
    DSU(int n) : p(n), r(n, 0) { iota(p.begin(), p.end(), 0); }
    int find(int x) { return p[x] == x ? x : p[x] = find(p[x]); }
    void unite(int a, int b) {
        a = find(a); b = find(b);
        if (a == b) return;
        if (r[a] < r[b]) swap(a, b);
        p[b] = a;
        if (r[a] == r[b]) r[a]++;
    }
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
        {0, {1, 2}}, {1, {0, 5}}, {2, {0}}, {3, {6}},
        {4, {}}, {5, {1}}, {6, {3}}
    };
    cout << countGroups(adj) << "\n"; // 3
    return 0;
}
