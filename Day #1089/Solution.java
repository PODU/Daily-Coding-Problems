// Sliding window maximum via monotonic decreasing deque of indices. Time O(n), Space O(k).
import java.util.*;

public class Solution {
    static List<Integer> maxSlidingWindow(int[] a, int k) {
        Deque<Integer> dq = new ArrayDeque<>(); // indices, values decreasing
        List<Integer> res = new ArrayList<>();
        for (int i = 0; i < a.length; i++) {
            if (!dq.isEmpty() && dq.peekFirst() <= i - k) dq.pollFirst();
            while (!dq.isEmpty() && a[dq.peekLast()] <= a[i]) dq.pollLast();
            dq.addLast(i);
            if (i >= k - 1) res.add(a[dq.peekFirst()]);
        }
        return res;
    }

    public static void main(String[] args) {
        int[] a = {10, 5, 2, 7, 8, 7};
        System.out.println(maxSlidingWindow(a, 3));
    }
}
