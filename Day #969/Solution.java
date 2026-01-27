// Day 969: Count islands (4-connected groups of 1s) in a binary matrix.
// Approach: DFS flood fill from each unvisited land cell. Time O(R*C), Space O(R*C).
public class Solution {
    static void dfs(int[][] g, int r, int c) {
        if (r < 0 || r >= g.length || c < 0 || c >= g[0].length || g[r][c] == 0) return;
        g[r][c] = 0;
        dfs(g, r + 1, c); dfs(g, r - 1, c); dfs(g, r, c + 1); dfs(g, r, c - 1);
    }

    static int numIslands(int[][] g) {
        int count = 0;
        for (int i = 0; i < g.length; i++)
            for (int j = 0; j < g[0].length; j++)
                if (g[i][j] == 1) { count++; dfs(g, i, j); }
        return count;
    }

    public static void main(String[] args) {
        int[][] g = {
            {1,0,0,0,0},
            {0,0,1,1,0},
            {0,1,1,0,0},
            {0,0,0,0,0},
            {1,1,0,0,1},
            {1,1,0,0,1}
        };
        System.out.println(numIslands(g)); // 4
    }
}
