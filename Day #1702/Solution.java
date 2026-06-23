// Largest rectangle of 1s: per-row histogram + largest-rectangle-in-histogram via stack.
// Time O(N*M), Space O(M).
import java.util.*;

public class Solution {
    static int largestInHist(int[] h) {
        int n = h.length, best = 0;
        Deque<Integer> st = new ArrayDeque<>();
        for (int i = 0; i <= n; i++) {
            int cur = (i == n) ? 0 : h[i];
            while (!st.isEmpty() && h[st.peek()] >= cur) {
                int height = h[st.pop()];
                int width = st.isEmpty() ? i : i - st.peek() - 1;
                best = Math.max(best, height * width);
            }
            st.push(i);
        }
        return best;
    }

    public static void main(String[] args) {
        int[][] mat = {{1,0,0,0},{1,0,1,1},{1,0,1,1},{0,1,0,0}};
        int n = mat.length, m = mat[0].length, best = 0;
        int[] h = new int[m];
        for (int i = 0; i < n; i++) {
            for (int j = 0; j < m; j++) h[j] = mat[i][j] == 1 ? h[j] + 1 : 0;
            best = Math.max(best, largestInHist(h));
        }
        System.out.println(best);
    }
}
