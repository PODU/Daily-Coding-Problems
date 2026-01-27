// Day 971: Rotate N x N matrix 90 degrees clockwise in place.
// Approach: transpose then reverse each row. Time O(N^2), Space O(1).
public class Solution {
    static void rotate(int[][] m) {
        int n = m.length;
        for (int i = 0; i < n; i++)
            for (int j = i + 1; j < n; j++) {
                int t = m[i][j]; m[i][j] = m[j][i]; m[j][i] = t;
            }
        for (int i = 0; i < n; i++)
            for (int l = 0, r = n - 1; l < r; l++, r--) {
                int t = m[i][l]; m[i][l] = m[i][r]; m[i][r] = t;
            }
    }

    public static void main(String[] args) {
        int[][] m = {{1,2,3},{4,5,6},{7,8,9}};
        rotate(m);
        for (int[] row : m) {
            StringBuilder sb = new StringBuilder();
            for (int j = 0; j < row.length; j++) {
                sb.append(row[j]);
                if (j + 1 < row.length) sb.append(" ");
            }
            System.out.println(sb);
        }
    }
}
