// Day 1500: Number of islands via DFS flood fill (4-directional).
// Time O(R*C), Space O(R*C) recursion worst case.
#include <bits/stdc++.h>
using namespace std;

void dfs(vector<vector<int>>& grid, int r, int c) {
    int R = grid.size(), C = grid[0].size();
    if (r < 0 || r >= R || c < 0 || c >= C || grid[r][c] == 0) return;
    grid[r][c] = 0;
    dfs(grid, r + 1, c);
    dfs(grid, r - 1, c);
    dfs(grid, r, c + 1);
    dfs(grid, r, c - 1);
}

int numIslands(vector<vector<int>> grid) {
    int count = 0;
    for (int r = 0; r < (int)grid.size(); r++)
        for (int c = 0; c < (int)grid[0].size(); c++)
            if (grid[r][c] == 1) { count++; dfs(grid, r, c); }
    return count;
}

int main() {
    vector<vector<int>> grid = {
        {1, 0, 0, 0, 0},
        {0, 0, 1, 1, 0},
        {0, 1, 1, 0, 0},
        {0, 0, 0, 0, 0},
        {1, 1, 0, 0, 1},
        {1, 1, 0, 0, 1}
    };
    cout << numIslands(grid) << endl;
    return 0;
}
