// Day 998: Graph k-colorability (adjacency matrix).
// Backtracking: try each color per vertex, skipping colors used by neighbors.
// O(k^V) worst case, O(V) extra space.
#include <bits/stdc++.h>
using namespace std;

vector<vector<int>> g;
vector<int> colors;
int n, K;

bool ok(int v, int c) {
    for (int u = 0; u < n; ++u)
        if (g[v][u] && colors[u] == c) return false;
    return true;
}

bool solve(int v) {
    if (v == n) return true;
    for (int c = 1; c <= K; ++c) {
        if (ok(v, c)) {
            colors[v] = c;
            if (solve(v + 1)) return true;
            colors[v] = 0;
        }
    }
    return false;
}

bool canColor(vector<vector<int>> graph, int k) {
    g = graph; n = graph.size(); K = k;
    colors.assign(n, 0);
    return solve(0);
}

int main() {
    vector<vector<int>> triangle = {{0,1,1},{1,0,1},{1,1,0}};
    cout << boolalpha;
    cout << canColor(triangle, 2) << "\n"; // false
    cout << canColor(triangle, 3) << "\n"; // true
    return 0;
}
