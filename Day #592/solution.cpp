// Day 592: Count islands in a binary matrix.
// Approach: DFS flood-fill over each unvisited land cell (4-directional).
// Time: O(R*C), Space: O(R*C) worst-case recursion stack.
#include <bits/stdc++.h>
using namespace std;

void dfs(vector<vector<int>>& g, int r, int c) {
    int R = g.size(), C = g[0].size();
    if (r < 0 || c < 0 || r >= R || c >= C || g[r][c] != 1) return;
    g[r][c] = 0; // sink
    dfs(g, r + 1, c);
    dfs(g, r - 1, c);
    dfs(g, r, c + 1);
    dfs(g, r, c - 1);
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
        {1, 0, 0, 0, 0},
        {0, 0, 1, 1, 0},
        {0, 1, 1, 0, 0},
        {0, 0, 0, 0, 0},
        {1, 1, 0, 0, 1},
        {1, 1, 0, 0, 1},
    };
    cout << numIslands(grid) << endl; // 4
    return 0;
}
