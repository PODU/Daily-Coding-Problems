// Day 302: Count regions divided by slashes. Split each cell into 4 triangles,
// union-find adjacent triangles. Time O(N*M*alpha), space O(N*M).
#include <bits/stdc++.h>
using namespace std;
struct DSU {
    vector<int> p;
    DSU(int n) : p(n) { iota(p.begin(), p.end(), 0); }
    int f(int x) { while (p[x] != x) { p[x] = p[p[x]]; x = p[x]; } return x; }
    void u(int a, int b) { p[f(a)] = f(b); }
};
int countRegions(vector<string> g) {
    int n = g.size(); size_t m = 0;
    for (auto& s : g) m = max(m, s.size());
    for (auto& s : g) s.resize(m, ' ');
    int M = m;
    DSU d(n * M * 4);
    auto id = [&](int i, int j, int t) { return (i * M + j) * 4 + t; }; // 0=top,1=right,2=bottom,3=left
    for (int i = 0; i < n; i++) for (int j = 0; j < M; j++) {
        char c = g[i][j];
        if (c == '/') { d.u(id(i,j,0), id(i,j,3)); d.u(id(i,j,1), id(i,j,2)); }
        else if (c == '\\') { d.u(id(i,j,0), id(i,j,1)); d.u(id(i,j,2), id(i,j,3)); }
        else { d.u(id(i,j,0), id(i,j,1)); d.u(id(i,j,1), id(i,j,2)); d.u(id(i,j,2), id(i,j,3)); }
        if (j + 1 < M) d.u(id(i,j,1), id(i,j+1,3));
        if (i + 1 < n) d.u(id(i,j,2), id(i+1,j,0));
    }
    int cnt = 0;
    for (int x = 0; x < n * M * 4; x++) if (d.f(x) == x) cnt++;
    return cnt;
}
int main() {
    vector<string> grid = {"\\    /", " \\  /", "  \\/"};
    cout << countRegions(grid) << "\n"; // 3
}
