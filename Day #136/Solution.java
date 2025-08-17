// Largest rectangle of 1's: per-row histogram + largest-rectangle-in-histogram via monotonic stack.
// Time O(N*M), Space O(M).
import java.util.*;

public class Solution {
    static int maximalRectangle(int[][] mat) {
        if (mat.length == 0 || mat[0].length == 0) return 0;
        int n = mat.length, m = mat[0].length, best = 0;
        int[] h = new int[m];
        for (int i = 0; i < n; i++) {
            for (int j = 0; j < m; j++) h[j] = mat[i][j] == 1 ? h[j] + 1 : 0;
            Deque<Integer> st = new ArrayDeque<>();
            for (int j = 0; j <= m; j++) {
                int cur = (j == m) ? 0 : h[j];
                while (!st.isEmpty() && h[st.peek()] >= cur) {
                    int height = h[st.pop()];
                    int width = st.isEmpty() ? j : j - st.peek() - 1;
                    best = Math.max(best, height * width);
                }
                st.push(j);
            }
        }
        return best;
    }

    public static void main(String[] args) {
        int[][] mat = {{1,0,0,0},{1,0,1,1},{1,0,1,1},{0,1,0,0}};
        System.out.println(maximalRectangle(mat));
    }
}
