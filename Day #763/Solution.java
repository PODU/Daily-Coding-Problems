// Day 763: Sliding window maximum via a monotonic decreasing deque of indices.
// Time: O(n), Space: O(k). Prints each window max as it is computed.
import java.util.*;

public class Solution {
    static void slidingMax(int[] a, int k) {
        Deque<Integer> dq = new ArrayDeque<>();   // indices, values decreasing
        List<Integer> out = new ArrayList<>();
        for (int i = 0; i < a.length; i++) {
            while (!dq.isEmpty() && a[dq.peekLast()] <= a[i]) dq.pollLast();
            dq.addLast(i);
            if (dq.peekFirst() <= i - k) dq.pollFirst();
            if (i >= k - 1) out.add(a[dq.peekFirst()]);
        }
        System.out.println(out);
    }

    public static void main(String[] args) {
        int[] a = {10, 5, 2, 7, 8, 7};
        slidingMax(a, 3);   // [10, 7, 8, 8]
    }
}
