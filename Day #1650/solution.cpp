// Maximum spanning tree: Kruskal with edges sorted DESC by weight + Union-Find (path compression, union by rank). O(E log E).
#include <iostream>
#include <vector>
#include <algorithm>

struct Edge { int u, v, w; };

struct DSU {
    std::vector<int> parent, rank_;
    DSU(int n) : parent(n), rank_(n, 0) {
        for (int i = 0; i < n; ++i) parent[i] = i;
    }
    int find(int x) {
        while (parent[x] != x) { parent[x] = parent[parent[x]]; x = parent[x]; }
        return x;
    }
    bool unite(int a, int b) {
        a = find(a); b = find(b);
        if (a == b) return false;
        if (rank_[a] < rank_[b]) std::swap(a, b);
        parent[b] = a;
        if (rank_[a] == rank_[b]) ++rank_[a];
        return true;
    }
};

int maxSpanningTree(int n, std::vector<Edge> edges) {
    std::sort(edges.begin(), edges.end(), [](const Edge& a, const Edge& b) {
        return a.w > b.w;
    });
    DSU dsu(n);
    int total = 0;
    for (const auto& e : edges)
        if (dsu.unite(e.u, e.v)) total += e.w;
    return total;
}

int main() {
    std::vector<Edge> edges = {{0,1,1},{0,2,2},{0,3,3},{1,2,4},{2,3,5}};
    std::cout << "Maximum spanning tree weight: " << maxSpanningTree(4, edges) << "\n";
    return 0;
}
