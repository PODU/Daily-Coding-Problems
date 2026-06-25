// Day 1716: Print an N x M matrix in clockwise spiral order.
// Boundary-shrinking traversal. Time: O(N*M), Space: O(1) extra.
public class Solution {
    static void spiral(int[][] m) {
        if (m.length == 0 || m[0].length == 0) return;
        int top = 0, bottom = m.length - 1;
        int left = 0, right = m[0].length - 1;
        StringBuilder sb = new StringBuilder();
        while (top <= bottom && left <= right) {
            for (int c = left; c <= right; c++) sb.append(m[top][c]).append("\n");
            top++;
            for (int r = top; r <= bottom; r++) sb.append(m[r][right]).append("\n");
            right--;
            if (top <= bottom) {
                for (int c = right; c >= left; c--) sb.append(m[bottom][c]).append("\n");
                bottom--;
            }
            if (left <= right) {
                for (int r = bottom; r >= top; r--) sb.append(m[r][left]).append("\n");
                left++;
            }
        }
        System.out.print(sb);
    }

    public static void main(String[] args) {
        int[][] m = {
            {1, 2, 3, 4, 5},
            {6, 7, 8, 9, 10},
            {11, 12, 13, 14, 15},
            {16, 17, 18, 19, 20}
        };
        spiral(m);
    }
}
