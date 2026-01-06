// Day 865: Minimum cost to connect all houses to the plant = MST cost.
// Approach: Kruskal's algorithm with union-find over all pipe edges.
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
    // pipes: plant->A 1, plant->B 5, plant->C 20, A->C 15, B->C 10
    map<string,int> id;
    auto getId = [&](const string& s){ if (!id.count(s)) id[s] = id.size(); return id[s]; };

    vector<tuple<int,int,int>> edges; // (cost, u, v)
    auto add = [&](string a, string b, int c){ edges.push_back({c, getId(a), getId(b)}); };
    add("plant","A",1); add("plant","B",5); add("plant","C",20);
    add("A","C",15); add("B","C",10);

    sort(edges.begin(), edges.end());
    DSU dsu(id.size());
    int total = 0;
    for (auto& [c, u, v] : edges)
        if (dsu.unite(u, v)) total += c;
    cout << total << endl; // 16
    return 0;
}
