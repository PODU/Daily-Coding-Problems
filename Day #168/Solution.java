// Rotate NxN 90 clockwise in place: transpose then reverse each row. O(n^2) time, O(1) extra space.
public class Solution {
    static void rotate(int[][] m) {
        int n = m.length;
        for (int i = 0; i < n; i++)
            for (int j = i + 1; j < n; j++) {
                int t = m[i][j]; m[i][j] = m[j][i]; m[j][i] = t;
            }
        for (int i = 0; i < n; i++)
            for (int a = 0, b = n - 1; a < b; a++, b--) {
                int t = m[i][a]; m[i][a] = m[i][b]; m[i][b] = t;
            }
    }

    public static void main(String[] args) {
        int[][] m = {{1, 2, 3}, {4, 5, 6}, {7, 8, 9}};
        rotate(m);
        for (int[] row : m) {
            StringBuilder sb = new StringBuilder("[");
            for (int j = 0; j < row.length; j++) {
                if (j > 0) sb.append(", ");
                sb.append(row[j]);
            }
            sb.append("]");
            System.out.println(sb.toString());
        }
    }
}
