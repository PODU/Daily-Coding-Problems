// Day 219: Connect 4 (7 cols x 6 rows).
// Approach: drop into lowest empty cell; after each move scan 4 directions from it for 4-in-a-row.
// Win check O(1) per move; board O(W*H).
public class Solution {
    static final int W = 7, H = 6;
    char[][] grid = new char[H][W];
    int[] height = new int[W];

    Solution() {
        for (char[] row : grid) java.util.Arrays.fill(row, '.');
    }

    int drop(int col, char player) {
        if (col < 0 || col >= W || height[col] >= H) return -1;
        int r = height[col]++;
        grid[r][col] = player;
        return r;
    }

    boolean wins(int r, int c, char p) {
        int[][] dirs = {{0, 1}, {1, 0}, {1, 1}, {1, -1}};
        for (int[] d : dirs) {
            int cnt = 1;
            for (int s = -1; s <= 1; s += 2) {
                int rr = r + d[0] * s, cc = c + d[1] * s;
                while (rr >= 0 && rr < H && cc >= 0 && cc < W && grid[rr][cc] == p) {
                    cnt++; rr += d[0] * s; cc += d[1] * s;
                }
            }
            if (cnt >= 4) return true;
        }
        return false;
    }

    public static void main(String[] args) {
        Solution g = new Solution();
        int[] cols = {0, 1, 0, 1, 0, 1, 0};
        char[] players = {'R', 'B', 'R', 'B', 'R', 'B', 'R'};
        char winner = ' ';
        for (int i = 0; i < cols.length; i++) {
            int row = g.drop(cols[i], players[i]);
            if (g.wins(row, cols[i], players[i])) { winner = players[i]; break; }
        }
        System.out.println("Player " + winner + " wins!"); // R wins vertically in column 0
    }
}
