// Day 1844: Largest rectangle in a histogram via a monotonic increasing stack.
// Time O(N), Space O(N).
import java.util.*;

public class Solution {
    static long largestRectangle(int[] heights) {
        int n = heights.length;
        int[] h = Arrays.copyOf(heights, n + 1); // sentinel 0 at end
        Deque<Integer> st = new ArrayDeque<>();
        long best = 0;
        for (int i = 0; i <= n; i++) {
            while (!st.isEmpty() && h[st.peek()] >= h[i]) {
                int height = h[st.pop()];
                int left = st.isEmpty() ? -1 : st.peek();
                long width = i - left - 1;
                best = Math.max(best, (long) height * width);
            }
            st.push(i);
        }
        return best;
    }

    public static void main(String[] args) {
        System.out.println(largestRectangle(new int[]{1, 3, 2, 5})); // 6
    }
}
