// Day 858: Running median of a stream.
// Approach: max-heap (lower half) + min-heap (upper half), balanced sizes.
// Time: O(n log n) total, Space: O(n).
import java.util.*;

public class Solution {
    static void printMedian(double m) {
        if (m == Math.floor(m)) System.out.println((long) m);
        else System.out.println(m);
    }

    public static void main(String[] args) {
        int[] stream = {2, 1, 5, 7, 2, 0, 5};
        PriorityQueue<Integer> lo = new PriorityQueue<>(Collections.reverseOrder());
        PriorityQueue<Integer> hi = new PriorityQueue<>();

        for (int x : stream) {
            if (lo.isEmpty() || x <= lo.peek()) lo.add(x);
            else hi.add(x);
            if (lo.size() > hi.size() + 1) hi.add(lo.poll());
            else if (hi.size() > lo.size()) lo.add(hi.poll());

            double med = lo.size() > hi.size() ? lo.peek() : (lo.peek() + hi.peek()) / 2.0;
            printMedian(med);
        }
    }
}
