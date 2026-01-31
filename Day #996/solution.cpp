// Day 996: Maximum weight spanning tree.
// Kruskal's algorithm with edges sorted in DESCENDING weight + union-find.
// O(E log E) time, O(V) space.
#include <bits/stdc++.h>
using namespace std;

struct DSU {
    vector<int> p;
    DSU(int n) : p(n) { iota(p.begin(), p.end(), 0); }
    int find(int x) { while (p[x] != x) { p[x] = p[p[x]]; x = p[x]; } return x; }
    bool unite(int a, int b) {
        int ra = find(a), rb = find(b);
        if (ra == rb) return false;
        p[ra] = rb; return true;
    }
};

int main() {
    int n = 4;
    // {weight, u, v}
    vector<array<int, 3>> edges = {{1,0,1},{3,0,2},{2,1,2},{4,1,3},{5,2,3}};
    sort(edges.rbegin(), edges.rend());          // heaviest first
    DSU dsu(n);
    int total = 0;
    for (auto& e : edges)
        if (dsu.unite(e[1], e[2])) total += e[0];
    cout << "Maximum spanning tree weight: " << total << "\n"; // 12
    return 0;
}
