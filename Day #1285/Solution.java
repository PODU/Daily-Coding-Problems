// Day 1285: Print a string in zigzag form across k lines.
// Place char i at column i, row = triangle-wave of i. Time O(n*k) to render, Space O(n*k).
public class Solution {
    static void zigzag(String s, int k) {
        int n = s.length();
        if (k <= 1) { System.out.println(s); return; }
        int period = 2 * (k - 1);
        char[][] grid = new char[k][n];
        for (char[] row : grid) java.util.Arrays.fill(row, ' ');
        for (int i = 0; i < n; ++i) {
            int pos = i % period;
            int row = (pos < k) ? pos : period - pos;
            grid[row][i] = s.charAt(i);
        }
        for (char[] row : grid) {
            String line = new String(row);
            int end = line.length();
            while (end > 0 && line.charAt(end - 1) == ' ') end--;
            System.out.println(line.substring(0, end));
        }
    }

    public static void main(String[] args) {
        zigzag("thisisazigzag", 4);
    }
}
