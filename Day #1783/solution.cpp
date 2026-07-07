// Count regions split by '/','\\',' ' via Union-Find: each cell = 4 triangles (T,R,B,L).
// Union within cell per char and across neighbors. Time O(R*C*alpha), Space O(R*C).
#include <bits/stdc++.h>
using namespace std;

struct DSU {
    vector<int> p;
    DSU(int n) : p(n) { iota(p.begin(), p.end(), 0); }
    int find(int x) { return p[x] == x ? x : p[x] = find(p[x]); }
    void uni(int a, int b) { p[find(a)] = find(b); }
};

int regions(const vector<string>& grid) {
    int R = grid.size(), C = grid[0].size();
    DSU dsu(4 * R * C);
    auto idx = [&](int r, int c, int t) { return 4 * (r * C + c) + t; };
    for (int r = 0; r < R; r++) {
        for (int c = 0; c < C; c++) {
            char ch = grid[r][c];
            // 0=top,1=right,2=bottom,3=left
            if (ch == ' ') {
                dsu.uni(idx(r, c, 0), idx(r, c, 1));
                dsu.uni(idx(r, c, 1), idx(r, c, 2));
                dsu.uni(idx(r, c, 2), idx(r, c, 3));
            } else if (ch == '/') {
                dsu.uni(idx(r, c, 0), idx(r, c, 3));
                dsu.uni(idx(r, c, 1), idx(r, c, 2));
            } else { // '\\'
                dsu.uni(idx(r, c, 0), idx(r, c, 1));
                dsu.uni(idx(r, c, 2), idx(r, c, 3));
            }
            if (c + 1 < C) dsu.uni(idx(r, c, 1), idx(r, c + 1, 3));
            if (r + 1 < R) dsu.uni(idx(r, c, 2), idx(r + 1, c, 0));
        }
    }
    set<int> roots;
    for (int i = 0; i < 4 * R * C; i++) roots.insert(dsu.find(i));
    return roots.size();
}

int main() {
    vector<string> grid = {
        "\\    /",
        " \\  / ",
        "  \\/  "
    };
    cout << regions(grid) << "\n"; // 3
    return 0;
}
