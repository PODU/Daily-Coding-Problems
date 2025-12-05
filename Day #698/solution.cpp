// Day 698: Count regions a grid of '/'/'\\'/' ' is divided into.
// Approach: split each cell into 4 triangles (top,right,bottom,left) and union by
// the slash type plus across neighbors (Union-Find). Time O(R*C*a), Space O(R*C).
#include <bits/stdc++.h>
using namespace std;

struct DSU {
    vector<int> p;
    DSU(int n) : p(n) { iota(p.begin(), p.end(), 0); }
    int find(int x) { return p[x] == x ? x : p[x] = find(p[x]); }
    void uni(int a, int b) { p[find(a)] = find(b); }
};

int regions(vector<string> grid) {
    int R = grid.size(), C = 0;
    for (auto& s : grid) C = max(C, (int)s.size());
    for (auto& s : grid) s.resize(C, ' ');
    DSU dsu(R * C * 4);
    auto id = [&](int r, int c, int k) { return (r * C + c) * 4 + k; }; // 0=T,1=R,2=B,3=L
    for (int r = 0; r < R; ++r)
        for (int c = 0; c < C; ++c) {
            char ch = grid[r][c];
            if (ch == '/') { dsu.uni(id(r,c,0), id(r,c,3)); dsu.uni(id(r,c,1), id(r,c,2)); }
            else if (ch == '\\') { dsu.uni(id(r,c,0), id(r,c,1)); dsu.uni(id(r,c,2), id(r,c,3)); }
            else { dsu.uni(id(r,c,0), id(r,c,1)); dsu.uni(id(r,c,1), id(r,c,2)); dsu.uni(id(r,c,2), id(r,c,3)); }
            if (c + 1 < C) dsu.uni(id(r,c,1), id(r,c+1,3));
            if (r + 1 < R) dsu.uni(id(r,c,2), id(r+1,c,0));
        }
    int cnt = 0;
    for (int i = 0; i < R * C * 4; ++i) if (dsu.find(i) == i) ++cnt;
    return cnt;
}

int main() {
    vector<string> grid = {"\\    /", " \\  /", "  \\/"};
    cout << regions(grid) << "\n"; // 3
    return 0;
}
