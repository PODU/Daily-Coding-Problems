// Day 84: Count islands (connected groups of 1s) via DFS flood fill.
// Time O(rows*cols), Space O(rows*cols) recursion worst case.
#include <bits/stdc++.h>
using namespace std;

void dfs(vector<vector<int>>& g, int r, int c) {
    if (r < 0 || c < 0 || r >= (int)g.size() || c >= (int)g[0].size() || g[r][c] == 0) return;
    g[r][c] = 0;
    dfs(g, r + 1, c); dfs(g, r - 1, c);
    dfs(g, r, c + 1); dfs(g, r, c - 1);
}

int numIslands(vector<vector<int>> g) {
    int count = 0;
    for (int r = 0; r < (int)g.size(); ++r)
        for (int c = 0; c < (int)g[0].size(); ++c)
            if (g[r][c] == 1) { ++count; dfs(g, r, c); }
    return count;
}

int main() {
    vector<vector<int>> grid = {
        {1,0,0,0,0},
        {0,0,1,1,0},
        {0,1,1,0,0},
        {0,0,0,0,0},
        {1,1,0,0,1},
        {1,1,0,0,1}};
    cout << numIslands(grid) << "\n"; // 4
    return 0;
}
