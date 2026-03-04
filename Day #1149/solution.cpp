// Day 1149: Number of islands - 4-directional flood fill.
// DFS marks visited land. Time O(R*C), Space O(R*C) worst-case recursion.
#include <bits/stdc++.h>
using namespace std;

void dfs(vector<vector<int>>& g, int r, int c) {
    if (r < 0 || c < 0 || r >= (int)g.size() || c >= (int)g[0].size() || g[r][c] == 0) return;
    g[r][c] = 0;
    dfs(g, r + 1, c); dfs(g, r - 1, c); dfs(g, r, c + 1); dfs(g, r, c - 1);
}

int numIslands(vector<vector<int>> g) {
    int count = 0;
    for (int r = 0; r < (int)g.size(); ++r)
        for (int c = 0; c < (int)g[0].size(); ++c)
            if (g[r][c] == 1) { count++; dfs(g, r, c); }
    return count;
}

int main() {
    vector<vector<int>> grid = {
        {1, 0, 0, 0, 0}, {0, 0, 1, 1, 0}, {0, 1, 1, 0, 0},
        {0, 0, 0, 0, 0}, {1, 1, 0, 0, 1}, {1, 1, 0, 0, 1}};
    cout << numIslands(grid) << "\n"; // 4
    return 0;
}
