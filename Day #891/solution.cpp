// Region cutting by slashes: split each cell into 4 triangles, union-find.
// Time: O(N*M * alpha), Space: O(N*M).
#include <bits/stdc++.h>
using namespace std;

vector<int> par;
int find(int x) { while (par[x] != x) { par[x] = par[par[x]]; x = par[x]; } return x; }
void uni(int a, int b) { par[find(a)] = find(b); }

int regions(vector<string>& g) {
    int n = g.size(), m = g[0].size();
    par.resize(n * m * 4);
    iota(par.begin(), par.end(), 0);
    auto idx = [&](int r, int c, int k) { return (r * m + c) * 4 + k; };
    for (int r = 0; r < n; r++) {
        for (int c = 0; c < m; c++) {
            char ch = g[r][c];
            if (ch == '/') { uni(idx(r,c,0), idx(r,c,3)); uni(idx(r,c,1), idx(r,c,2)); }
            else if (ch == '\\') { uni(idx(r,c,0), idx(r,c,1)); uni(idx(r,c,2), idx(r,c,3)); }
            else { uni(idx(r,c,0), idx(r,c,1)); uni(idx(r,c,1), idx(r,c,2)); uni(idx(r,c,2), idx(r,c,3)); }
            if (c + 1 < m) uni(idx(r,c,1), idx(r,c+1,3));
            if (r + 1 < n) uni(idx(r,c,2), idx(r+1,c,0));
        }
    }
    int cnt = 0;
    for (int i = 0; i < n * m * 4; i++) if (find(i) == i) cnt++;
    return cnt;
}

int main() {
    vector<string> g = {
        "\\    /",
        " \\  / ",
        "  \\/  "
    };
    cout << regions(g) << "\n";
    return 0;
}
