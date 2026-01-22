// Day 939: Print an N x M matrix in clockwise spiral order.
// Shrink four boundaries layer by layer. Time O(N*M), Space O(1) extra.
public class Solution {
    static void spiral(int[][] m) {
        if (m.length == 0) return;
        int top = 0, bottom = m.length - 1, left = 0, right = m[0].length - 1;
        while (top <= bottom && left <= right) {
            for (int c = left; c <= right; c++) System.out.println(m[top][c]);
            top++;
            for (int r = top; r <= bottom; r++) System.out.println(m[r][right]);
            right--;
            if (top <= bottom) {
                for (int c = right; c >= left; c--) System.out.println(m[bottom][c]);
                bottom--;
            }
            if (left <= right) {
                for (int r = bottom; r >= top; r--) System.out.println(m[r][left]);
                left++;
            }
        }
    }

    public static void main(String[] args) {
        int[][] m = {
            {1, 2, 3, 4, 5},
            {6, 7, 8, 9, 10},
            {11, 12, 13, 14, 15},
            {16, 17, 18, 19, 20}};
        spiral(m); // 1 2 3 4 5 10 15 20 19 18 17 16 11 6 7 8 9 14 13 12
    }
}
