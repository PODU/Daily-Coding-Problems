// Day 1149: Number of islands - 4-directional flood fill.
// DFS marks visited land. Time O(R*C), Space O(R*C) worst-case recursion.
public class Solution {
    static void dfs(int[][] g, int r, int c) {
        if (r < 0 || c < 0 || r >= g.length || c >= g[0].length || g[r][c] == 0) return;
        g[r][c] = 0;
        dfs(g, r + 1, c); dfs(g, r - 1, c); dfs(g, r, c + 1); dfs(g, r, c - 1);
    }

    static int numIslands(int[][] g) {
        int count = 0;
        for (int r = 0; r < g.length; r++)
            for (int c = 0; c < g[0].length; c++)
                if (g[r][c] == 1) { count++; dfs(g, r, c); }
        return count;
    }

    public static void main(String[] args) {
        int[][] grid = {
            {1, 0, 0, 0, 0}, {0, 0, 1, 1, 0}, {0, 1, 1, 0, 0},
            {0, 0, 0, 0, 0}, {1, 1, 0, 0, 1}, {1, 1, 0, 0, 1}};
        System.out.println(numIslands(grid)); // 4
    }
}
