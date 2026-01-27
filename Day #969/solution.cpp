// Day 969: Count islands (4-connected groups of 1s) in a binary matrix.
// Approach: DFS flood fill from each unvisited land cell. Time O(R*C), Space O(R*C).
#include <bits/stdc++.h>
using namespace std;

void dfs(vector<vector<int>>& g, int r, int c) {
    if (r < 0 || r >= (int)g.size() || c < 0 || c >= (int)g[0].size() || g[r][c] == 0) return;
    g[r][c] = 0;
    dfs(g, r + 1, c); dfs(g, r - 1, c); dfs(g, r, c + 1); dfs(g, r, c - 1);
}

int numIslands(vector<vector<int>> g) {
    int count = 0;
    for (int i = 0; i < (int)g.size(); ++i)
        for (int j = 0; j < (int)g[0].size(); ++j)
            if (g[i][j] == 1) { count++; dfs(g, i, j); }
    return count;
}

int main() {
    vector<vector<int>> g = {
        {1,0,0,0,0},
        {0,0,1,1,0},
        {0,1,1,0,0},
        {0,0,0,0,0},
        {1,1,0,0,1},
        {1,1,0,0,1}
    };
    cout << numIslands(g) << endl; // 4
    return 0;
}
