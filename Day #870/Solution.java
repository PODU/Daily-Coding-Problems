// Day 870: Largest rectangle of 1's in a binary matrix.
// Approach: build per-row histogram of consecutive 1's, apply largest-rectangle-in-histogram.
// Time: O(N*M), Space: O(M).
import java.util.*;

public class Solution {
    static int largestInHist(int[] h) {
        Deque<Integer> st = new ArrayDeque<>();
        int best = 0, n = h.length;
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

    static int maximalRectangle(int[][] mat) {
        if (mat.length == 0) return 0;
        int m = mat[0].length;
        int[] h = new int[m];
        int best = 0;
        for (int[] row : mat) {
            for (int j = 0; j < m; j++) h[j] = row[j] == 1 ? h[j] + 1 : 0;
            best = Math.max(best, largestInHist(h));
        }
        return best;
    }

    public static void main(String[] args) {
        int[][] mat = {
            {1,0,0,0},
            {1,0,1,1},
            {1,0,1,1},
            {0,1,0,0}
        };
        System.out.println(maximalRectangle(mat)); // 4
    }
}
