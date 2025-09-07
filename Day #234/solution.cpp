// Maximum spanning tree: Kruskal with edges sorted by descending weight + union-find.
// Time: O(E log E), Space: O(V).
#include <bits/stdc++.h>
using namespace std;

struct DSU {
    vector<int> p;
    DSU(int n) : p(n) { iota(p.begin(), p.end(), 0); }
    int find(int x) { return p[x] == x ? x : p[x] = find(p[x]); }
    bool unite(int a, int b) {
        a = find(a); b = find(b);
        if (a == b) return false;
        p[a] = b;
        return true;
    }
};

int maxSpanningTree(int n, vector<array<int, 3>> edges, vector<array<int, 2>>& chosen) {
    sort(edges.begin(), edges.end(), [](auto& a, auto& b) { return a[2] > b[2]; });
    DSU dsu(n);
    int total = 0;
    for (auto& e : edges) {
        if (dsu.unite(e[0], e[1])) {
            total += e[2];
            chosen.push_back({e[0], e[1]});
        }
    }
    return total;
}

int main() {
    int n = 4;
    vector<array<int, 3>> edges = {{0, 1, 1}, {1, 2, 2}, {2, 3, 3}, {0, 3, 4}, {0, 2, 5}};
    vector<array<int, 2>> chosen;
    int total = maxSpanningTree(n, edges, chosen);
    cout << "Max spanning tree weight: " << total << "\n"; // 11
    cout << "Edges: ";
    for (auto& e : chosen) cout << "(" << e[0] << "-" << e[1] << ") ";
    cout << "\n";
}
