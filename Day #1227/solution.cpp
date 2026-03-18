// Graph k-colorability via backtracking: assign colors 1..k to vertices in order,
// skipping conflicts. Time O(k^n) worst case, Space O(n).
#include <bits/stdc++.h>
using namespace std;

bool safe(int v, const vector<vector<int>>& g, vector<int>& color, int c) {
    for (int i = 0; i < (int)g.size(); i++)
        if (g[v][i] && color[i] == c) return false;
    return true;
}

bool colorize(int v, const vector<vector<int>>& g, int k, vector<int>& color) {
    int n = g.size();
    if (v == n) return true;
    for (int c = 1; c <= k; c++) {
        if (safe(v, g, color, c)) {
            color[v] = c;
            if (colorize(v + 1, g, k, color)) return true;
            color[v] = 0;
        }
    }
    return false;
}

bool isKColorable(const vector<vector<int>>& g, int k) {
    vector<int> color(g.size(), 0);
    return colorize(0, g, k, color);
}

int main() {
    vector<vector<int>> g = {{0,1,1},{1,0,1},{1,1,0}};
    cout << (isKColorable(g, 2) ? "true" : "false") << "\n";
    cout << (isKColorable(g, 3) ? "true" : "false") << "\n";
    return 0;
}
