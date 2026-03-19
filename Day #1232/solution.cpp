// Count regions carved by slashes. Split each cell into 4 triangles + Union-Find.
// Time O(n*m a(n*m)), Space O(n*m).
#include <bits/stdc++.h>
using namespace std;

struct DSU {
    vector<int> p;
    int count;
    DSU(int n) : p(n), count(n) { iota(p.begin(), p.end(), 0); }
    int find(int x) { while (p[x] != x) { p[x] = p[p[x]]; x = p[x]; } return x; }
    void unite(int a, int b) { int ra = find(a), rb = find(b); if (ra != rb) { p[ra] = rb; --count; } }
};

int regions(const vector<string>& grid) {
    int rows = grid.size(), cols = rows ? grid[0].size() : 0;
    DSU dsu(4 * rows * cols);
    auto idx = [&](int r, int c, int k) { return 4 * (r * cols + c) + k; };
    for (int r = 0; r < rows; ++r)
        for (int c = 0; c < cols; ++c) {
            char ch = grid[r][c];
            int t = idx(r, c, 0), ri = idx(r, c, 1), b = idx(r, c, 2), le = idx(r, c, 3);
            if (ch == '/') { dsu.unite(t, le); dsu.unite(ri, b); }
            else if (ch == '\\') { dsu.unite(t, ri); dsu.unite(le, b); }
            else { dsu.unite(t, ri); dsu.unite(ri, b); dsu.unite(b, le); }
            if (c + 1 < cols) dsu.unite(ri, idx(r, c + 1, 3));
            if (r + 1 < rows) dsu.unite(b, idx(r + 1, c, 0));
        }
    return dsu.count;
}

int main() {
    vector<string> grid = {"\\    /", " \\  / ", "  \\/  "};
    printf("%d\n", regions(grid));
    return 0;
}
