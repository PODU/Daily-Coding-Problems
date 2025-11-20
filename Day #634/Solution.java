// Day 634: Largest rectangle in a histogram.
// Approach: monotonic increasing stack of bar indices; pop to settle areas.
// Time: O(N), Space: O(N).
import java.util.*;

public class Solution {
    static long largestRectangle(int[] h) {
        int n = h.length;
        Deque<Integer> st = new ArrayDeque<>();
        long best = 0;
        for (int i = 0; i <= n; i++) {
            int cur = (i == n) ? 0 : h[i];
            while (!st.isEmpty() && h[st.peek()] >= cur) {
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
        int[] hist = {1, 3, 2, 5};
        System.out.println(largestRectangle(hist)); // 6
    }
}
