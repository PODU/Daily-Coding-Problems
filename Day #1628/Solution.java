// Day 1628: Rotate N x N matrix 90 degrees clockwise in place.
// Transpose then reverse each row. Time O(N^2), Space O(1).
public class Solution {
    static void rotate(int[][] m) {
        int n = m.length;
        for (int i = 0; i < n; i++)
            for (int j = i + 1; j < n; j++) {
                int t = m[i][j]; m[i][j] = m[j][i]; m[j][i] = t;
            }
        for (int[] row : m)
            for (int a = 0, b = n - 1; a < b; a++, b--) {
                int t = row[a]; row[a] = row[b]; row[b] = t;
            }
    }

    public static void main(String[] args) {
        int[][] m = {{1, 2, 3}, {4, 5, 6}, {7, 8, 9}};
        rotate(m);
        StringBuilder sb = new StringBuilder();
        for (int[] row : m) {
            for (int j = 0; j < row.length; j++) sb.append(row[j]).append(j + 1 < row.length ? " " : "");
            sb.append("\n");
        }
        System.out.print(sb);
    }
}
