// Day 280: Detect cycle in undirected graph via Union-Find.
// An edge joining two already-connected vertices implies a cycle.
// Time O(V + E * alpha(V)), Space O(V).
#include <bits/stdc++.h>
using namespace std;

struct DSU {
    vector<int> p;
    DSU(int n) : p(n) { iota(p.begin(), p.end(), 0); }
    int find(int x) { return p[x] == x ? x : p[x] = find(p[x]); }
    bool unite(int a, int b) {
        a = find(a); b = find(b);
        if (a == b) return false; // already connected -> cycle
        p[a] = b;
        return true;
    }
};

bool hasCycle(int n, vector<pair<int,int>>& edges) {
    DSU dsu(n);
    for (auto& [u, v] : edges)
        if (!dsu.unite(u, v)) return true;
    return false;
}

int main() {
    int n = 4;
    vector<pair<int,int>> edges = {{0,1},{1,2},{2,0},{2,3}};
    cout << boolalpha << hasCycle(n, edges) << "\n"; // true
    vector<pair<int,int>> tree = {{0,1},{1,2},{2,3}};
    cout << boolalpha << hasCycle(n, tree) << "\n";  // false
    return 0;
}
