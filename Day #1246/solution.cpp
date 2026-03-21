// Day 1246: Maximum weight spanning tree.
// Approach: Kruskal's algorithm with edges sorted in DESCENDING weight + union-find.
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

// edges: {weight, u, v}
long long maxSpanningTree(int n, vector<array<int,3>> edges) {
    sort(edges.begin(), edges.end(), [](const array<int,3>& a, const array<int,3>& b){
        return a[0] > b[0];
    });
    DSU dsu(n);
    long long total = 0;
    for (auto& e : edges)
        if (dsu.unite(e[1], e[2])) total += e[0];
    return total;
}

int main() {
    int n = 4;
    vector<array<int,3>> edges = {{1,0,1},{2,1,2},{3,2,3},{4,0,3},{5,0,2}};
    cout << maxSpanningTree(n, edges) << "\n"; // 11
    return 0;
}
