// Day 1438: Largest rectangle in a histogram.
// Approach: monotonic increasing stack of bar indices; pop to compute areas.
// Time: O(n), Space: O(n).
import java.util.ArrayDeque;
import java.util.Deque;

public class Solution {
    static long largestRectangle(int[] heights) {
        Deque<Integer> st = new ArrayDeque<>();
        long best = 0;
        int n = heights.length;
        for (int i = 0; i <= n; i++) {
            int h = (i == n) ? 0 : heights[i];
            while (!st.isEmpty() && heights[st.peek()] >= h) {
                int top = st.pop();
                int width = st.isEmpty() ? i : i - st.peek() - 1;
                best = Math.max(best, (long) heights[top] * width);
            }
            st.push(i);
        }
        return best;
    }

    public static void main(String[] args) {
        int[] h = {1, 3, 2, 5};
        System.out.println(largestRectangle(h)); // 6
    }
}
