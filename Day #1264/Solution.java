// Day 1264: Largest rectangle of 1's in a binary matrix.
// Per-row histogram + largest-rectangle-in-histogram via monotonic stack. O(N*M) time, O(M) space.
import java.util.*;

public class Solution {
    static int largestInHistogram(int[] h) {
        int n = h.length, best = 0;
        Deque<Integer> st = new ArrayDeque<>();
        for (int i = 0; i <= n; i++) {
            int cur = (i == n) ? 0 : h[i];
            while (!st.isEmpty() && h[st.peek()] >= cur) {
                int height = h[st.pop()];
                int left = st.isEmpty() ? -1 : st.peek();
                best = Math.max(best, height * (i - left - 1));
            }
            st.push(i);
        }
        return best;
    }

    static int maximalRectangle(int[][] m) {
        if (m.length == 0) return 0;
        int cols = m[0].length, best = 0;
        int[] h = new int[cols];
        for (int[] row : m) {
            for (int j = 0; j < cols; j++) h[j] = row[j] == 1 ? h[j] + 1 : 0;
            best = Math.max(best, largestInHistogram(h));
        }
        return best;
    }

    public static void main(String[] args) {
        int[][] m = {{1,0,0,0},{1,0,1,1},{1,0,1,1},{0,1,0,0}};
        System.out.println(maximalRectangle(m));
    }
}
