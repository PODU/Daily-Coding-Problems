// Spiral order print via boundary shrinking (top/bottom/left/right). Time O(N*M), Space O(1) extra.
public class Solution {
    public static void main(String[] args) {
        int[][] m = {
            {1,2,3,4,5},
            {6,7,8,9,10},
            {11,12,13,14,15},
            {16,17,18,19,20}
        };
        int n = m.length, cols = m[0].length;
        int top = 0, bottom = n - 1, left = 0, right = cols - 1;
        while (top <= bottom && left <= right) {
            for (int j = left; j <= right; j++) System.out.println(m[top][j]);
            top++;
            for (int i = top; i <= bottom; i++) System.out.println(m[i][right]);
            right--;
            if (top <= bottom) {
                for (int j = right; j >= left; j--) System.out.println(m[bottom][j]);
                bottom--;
            }
            if (left <= right) {
                for (int i = bottom; i >= top; i--) System.out.println(m[i][left]);
                left++;
            }
        }
    }
}
