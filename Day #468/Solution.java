// Rotate NxN matrix 90 deg clockwise in place: transpose then reverse each row.
// Time: O(n^2), Space: O(1).
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
        int n = m.length;
        StringBuilder sb = new StringBuilder();
        for (int i = 0; i < n; i++) {
            sb.append(i == 0 ? "[[" : " [");
            for (int j = 0; j < n; j++) {
                sb.append(m[i][j]);
                if (j < n - 1) sb.append(", ");
            }
            sb.append("]");
            sb.append(i < n - 1 ? ",\n" : "]");
        }
        System.out.println(sb.toString());
    }
}
