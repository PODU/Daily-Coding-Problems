// Largest rectangle in histogram via monotonic increasing stack. Time O(N), Space O(N).
import java.util.*;

public class Solution {
    static long largestRectangle(int[] heights) {
        Deque<Integer> st = new ArrayDeque<>(); // indices of increasing bars
        long best = 0;
        int n = heights.length;
        for (int i = 0; i <= n; i++) {
            int h = (i < n) ? heights[i] : 0;
            while (!st.isEmpty() && heights[st.peek()] >= h) {
                int height = heights[st.pop()];
                int width = st.isEmpty() ? i : i - st.peek() - 1;
                best = Math.max(best, (long) height * width);
            }
            st.push(i);
        }
        return best;
    }

    public static void main(String[] args) {
        int[] heights = {1, 3, 2, 5};
        System.out.println(largestRectangle(heights));
    }
}
