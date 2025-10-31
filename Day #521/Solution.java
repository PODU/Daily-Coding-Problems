// Zigzag: place char i at (zigzag-row, column i) in a k x n grid; print rows. O(n*k).
public class Solution {
    static void zigzag(String s, int k) {
        int n = s.length();
        char[][] grid = new char[k][n];
        for (char[] row : grid) java.util.Arrays.fill(row, ' ');
        int row = 0, dir = (k == 1 ? 0 : 1);
        for (int i = 0; i < n; i++) {
            grid[row][i] = s.charAt(i);
            if (row == 0) dir = 1;
            else if (row == k - 1) dir = -1;
            row += dir;
        }
        StringBuilder out = new StringBuilder();
        for (char[] r : grid) {
            String line = new String(r);
            int end = line.length();
            while (end > 0 && line.charAt(end - 1) == ' ') end--;
            out.append(line, 0, end).append('\n');
        }
        System.out.print(out);
    }

    public static void main(String[] args) {
        zigzag("thisisazigzag", 4);
    }
}
