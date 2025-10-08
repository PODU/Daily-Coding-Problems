// Island perimeter: +4 per land cell, -2 per adjacent right/down land pair. O(R*C) time, O(1) space.
public class Solution {
    static int islandPerimeter(int[][] g) {
        int R = g.length, C = R == 0 ? 0 : g[0].length, per = 0;
        for (int r = 0; r < R; r++)
            for (int c = 0; c < C; c++)
                if (g[r][c] == 1) {
                    per += 4;
                    if (c + 1 < C && g[r][c + 1] == 1) per -= 2;
                    if (r + 1 < R && g[r + 1][c] == 1) per -= 2;
                }
        return per;
    }

    public static void main(String[] args) {
        int[][] grid = {{0,1,1,0},{1,1,1,0},{0,1,1,0},{0,0,1,0}};
        System.out.println(islandPerimeter(grid));
    }
}
