// Day 721: Maximum-weight spanning tree.
// Approach: Kruskal with edges sorted by DECREASING weight + union-find.
// Time: O(E log E), Space: O(V + E).
#include <bits/stdc++.h>
using namespace std;

struct DSU {
    vector<int> p, r;
    DSU(int n): p(n), r(n, 0) { iota(p.begin(), p.end(), 0); }
    int find(int x) { return p[x] == x ? x : p[x] = find(p[x]); }
    bool unite(int a, int b) {
        a = find(a); b = find(b);
        if (a == b) return false;
        if (r[a] < r[b]) swap(a, b);
        p[b] = a;
        if (r[a] == r[b]) r[a]++;
        return true;
    }
};

long long maxSpanningTree(int n, vector<array<int,3>> edges) {
    sort(edges.begin(), edges.end(), [](auto& a, auto& b){ return a[2] > b[2]; });
    DSU dsu(n);
    long long total = 0; int used = 0;
    for (auto& e : edges)
        if (dsu.unite(e[0], e[1])) { total += e[2]; used++; }
    return used == n - 1 ? total : -1; // -1 if disconnected
}

int main() {
    int n = 4;
    vector<array<int,3>> edges = {{0,1,1},{0,2,2},{0,3,3},{1,2,4},{2,3,5}};
    cout << "Maximum spanning tree weight: " << maxSpanningTree(n, edges) << "\n";
    return 0;
}
