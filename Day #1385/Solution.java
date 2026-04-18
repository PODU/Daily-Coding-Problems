// Spiral matrix traversal using four boundary pointers (top,bottom,left,right). O(N*M) time, O(1) extra space.
public class Solution {
    public static void main(String[] args) {
        int[][] mat = {
            {1,2,3,4,5},
            {6,7,8,9,10},
            {11,12,13,14,15},
            {16,17,18,19,20}
        };
        int top = 0, bottom = mat.length - 1;
        int left = 0, right = mat[0].length - 1;
        StringBuilder sb = new StringBuilder();
        while (top <= bottom && left <= right) {
            for (int j = left; j <= right; j++) sb.append(mat[top][j]).append("\n");
            top++;
            for (int i = top; i <= bottom; i++) sb.append(mat[i][right]).append("\n");
            right--;
            if (top <= bottom) {
                for (int j = right; j >= left; j--) sb.append(mat[bottom][j]).append("\n");
                bottom--;
            }
            if (left <= right) {
                for (int i = bottom; i >= top; i--) sb.append(mat[i][left]).append("\n");
                left++;
            }
        }
        System.out.print(sb);
    }
}
