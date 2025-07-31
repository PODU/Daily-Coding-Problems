// Day 56: Graph k-colorability via backtracking.
// Time: O(k^V) worst case, Space: O(V).
#include <bits/stdc++.h>
using namespace std;

bool canColor(const vector<vector<int>>& g, int k, vector<int>& color, int v) {
    int n = g.size();
    if (v == n) return true;
    for (int c = 1; c <= k; c++) {
        bool ok = true;
        for (int u = 0; u < n; u++)
            if (g[v][u] && color[u] == c) { ok = false; break; }
        if (!ok) continue;
        color[v] = c;
        if (canColor(g, k, color, v + 1)) return true;
        color[v] = 0;
    }
    return false;
}

bool kColorable(const vector<vector<int>>& g, int k) {
    vector<int> color(g.size(), 0);
    return canColor(g, k, color, 0);
}

int main() {
    // Triangle graph (3 mutually adjacent vertices): needs 3 colors.
    vector<vector<int>> g = {
        {0,1,1},
        {1,0,1},
        {1,1,0}
    };
    cout << boolalpha << kColorable(g, 2) << endl; // false
    cout << boolalpha << kColorable(g, 3) << endl; // true
    return 0;
}
