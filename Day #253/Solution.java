// Zigzag print: place each char at advancing column, row bounces 0..k-1..0.
// Build k row buffers, fill columns; rtrim each row. Time O(n*k), Space O(n*k).
public class Solution {
    static String[] zigzag(String s, int k) {
        int n = s.length();
        char[][] grid = new char[k][n];
        for (char[] r : grid) java.util.Arrays.fill(r, ' ');
        int row = 0, dir = 1;
        for (int col = 0; col < n; col++) {
            grid[row][col] = s.charAt(col);
            if (k > 1) {
                if (row == 0) dir = 1;
                else if (row == k - 1) dir = -1;
                row += dir;
            }
        }
        String[] out = new String[k];
        for (int i = 0; i < k; i++) {
            out[i] = new String(grid[i]).replaceAll("\\s+$", "");
        }
        return out;
    }

    public static void main(String[] args) {
        for (String r : zigzag("thisisazigzag", 4)) System.out.println(r);
    }
}
