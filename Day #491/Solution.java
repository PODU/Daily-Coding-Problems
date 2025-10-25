// Day 491: Running median of a stream.
// Two heaps: a max-heap keeps the lower half, a min-heap the upper half; rebalance so
// the lower half has equal size or one extra, so the median is its top.
// Time: O(log n) per element, Space: O(n).
import java.util.Collections;
import java.util.PriorityQueue;

public class Solution {
    public static void main(String[] args) {
        int[] stream = {2, 1, 5, 7, 2, 0, 5};
        PriorityQueue<Integer> lo = new PriorityQueue<>(Collections.reverseOrder()); // lower half
        PriorityQueue<Integer> hi = new PriorityQueue<>();                           // upper half
        for (int x : stream) {
            if (lo.isEmpty() || x <= lo.peek()) lo.offer(x); else hi.offer(x);
            if (lo.size() > hi.size() + 1) hi.offer(lo.poll());
            else if (hi.size() > lo.size()) lo.offer(hi.poll());
            double m = (lo.size() == hi.size()) ? (lo.peek() + hi.peek()) / 2.0 : lo.peek();
            // 2, 1.5, 2, 3.5, 2, 2, 2
            System.out.println(m == Math.floor(m) ? String.valueOf((long) m) : String.valueOf(m));
        }
    }
}
