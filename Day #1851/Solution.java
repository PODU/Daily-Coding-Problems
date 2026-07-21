// Day 1851: Zigzag string across k lines.
// Place char i at row=zigzag(i), col=i in a grid; print rows. O(n*k) build. Space O(n*k).
public class Solution {
    static String[] zigzag(String s, int k) {
        int n = s.length();
        char[][] grid = new char[k][n];
        for (char[] row : grid) java.util.Arrays.fill(row, ' ');
        int period = (k <= 1) ? 1 : 2 * (k - 1);
        for (int i = 0; i < n; i++) {
            int pos = (k <= 1) ? 0 : i % period;
            int row = (pos < k) ? pos : period - pos;
            grid[row][i] = s.charAt(i);
        }
        String[] out = new String[k];
        for (int r = 0; r < k; r++) {
            String line = new String(grid[r]);
            int e = line.length();
            while (e > 0 && line.charAt(e - 1) == ' ') e--;
            out[r] = line.substring(0, e);
        }
        return out;
    }

    public static void main(String[] args) {
        for (String line : zigzag("thisisazigzag", 4)) System.out.println(line);
    }
}
