// Running median with two heaps (max-heap lower half, min-heap upper half). O(log n) per insert.
import java.util.*;

public class Solution {
    public static void main(String[] args) {
        int[] stream = {2, 1, 5, 7, 2, 0, 5};
        PriorityQueue<Integer> lo = new PriorityQueue<>(Collections.reverseOrder()); // lower half
        PriorityQueue<Integer> hi = new PriorityQueue<>();                            // upper half
        StringBuilder sb = new StringBuilder();
        for (int x : stream) {
            if (lo.isEmpty() || x <= lo.peek()) lo.add(x); else hi.add(x);
            if (lo.size() > hi.size() + 1) hi.add(lo.poll());
            else if (hi.size() > lo.size()) lo.add(hi.poll());
            double m = (lo.size() == hi.size()) ? (lo.peek() + hi.peek()) / 2.0 : lo.peek();
            if (m == Math.floor(m)) sb.append((long) m).append("\n");
            else sb.append(m).append("\n");
        }
        System.out.print(sb);
    }
}
