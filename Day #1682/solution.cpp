// Day 1682: Graph k-colorability via backtracking with pruning.
// Time O(k^V) worst case, Space O(V).
#include <bits/stdc++.h>
using namespace std;

bool canColor(const vector<vector<int>>& adj, int k) {
    int n = adj.size();
    vector<int> color(n, -1);
    function<bool(int)> solve = [&](int v) -> bool {
        if (v == n) return true;
        for (int c = 0; c < k; ++c) {
            bool ok = true;
            for (int u = 0; u < n; ++u)
                if (adj[v][u] && color[u] == c) { ok = false; break; }
            if (ok) {
                color[v] = c;
                if (solve(v + 1)) return true;
                color[v] = -1;
            }
        }
        return false;
    };
    return solve(0);
}

int main() {
    // triangle: needs 3 colors
    vector<vector<int>> adj = {{0,1,1},{1,0,1},{1,1,0}};
    cout << (canColor(adj, 2) ? "True" : "False") << "\n"; // False
    cout << (canColor(adj, 3) ? "True" : "False") << "\n"; // True
    return 0;
}
