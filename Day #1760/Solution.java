// Day 1760: Sliding window maximum.
// Approach: monotonic deque of indices (decreasing values). O(n) time, O(k) space.
import java.util.*;

public class Solution {
    static int[] maxSlidingWindow(int[] a, int k) {
        int n = a.length;
        int[] res = new int[n - k + 1];
        Deque<Integer> dq = new ArrayDeque<>(); // indices, values decreasing
        int idx = 0;
        for (int i = 0; i < n; i++) {
            if (!dq.isEmpty() && dq.peekFirst() <= i - k) dq.pollFirst();
            while (!dq.isEmpty() && a[dq.peekLast()] <= a[i]) dq.pollLast();
            dq.offerLast(i);
            if (i >= k - 1) res[idx++] = a[dq.peekFirst()];
        }
        return res;
    }

    public static void main(String[] args) {
        int[] a = {10, 5, 2, 7, 8, 7};
        int k = 3;
        int[] res = maxSlidingWindow(a, k);
        StringBuilder sb = new StringBuilder("[");
        for (int i = 0; i < res.length; i++) {
            sb.append(res[i]);
            if (i + 1 < res.length) sb.append(", ");
        }
        sb.append("]");
        System.out.println(sb);
    }
}
