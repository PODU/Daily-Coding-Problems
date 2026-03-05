// Day 1155: Sliding window maximum via monotonic decreasing deque of indices.
// Each index pushed/popped once. Time O(n), Space O(k).
import java.util.*;

public class Solution {
    static int[] maxWindow(int[] a, int k) {
        Deque<Integer> dq = new ArrayDeque<>();
        int[] res = new int[a.length - k + 1];
        int ri = 0;
        for (int i = 0; i < a.length; i++) {
            if (!dq.isEmpty() && dq.peekFirst() <= i - k) dq.pollFirst();
            while (!dq.isEmpty() && a[dq.peekLast()] <= a[i]) dq.pollLast();
            dq.addLast(i);
            if (i >= k - 1) res[ri++] = a[dq.peekFirst()];
        }
        return res;
    }

    public static void main(String[] args) {
        int[] a = {10, 5, 2, 7, 8, 7};
        System.out.println(Arrays.toString(maxWindow(a, 3))); // [10, 7, 8, 8]
    }
}
