// Day 504: Last-N order log via fixed-size circular buffer.
// record O(1), get_last(i) O(1); space O(N).
public class Solution {
    static class OrderLog {
        private final long[] buf;
        private final int cap;
        private int pos = 0;   // next write index
        private int count = 0; // records seen (capped at cap)

        OrderLog(int n) {
            cap = n;
            buf = new long[n];
        }

        void record(long orderId) {
            buf[pos] = orderId;
            pos = (pos + 1) % cap;
            if (count < cap) count++;
        }

        // i = 1 is the most recent.
        long getLast(int i) {
            int idx = ((pos - i) % cap + cap) % cap;
            return buf[idx];
        }
    }

    public static void main(String[] args) {
        OrderLog log = new OrderLog(5);
        for (long id : new long[]{1, 2, 3, 4, 5, 6, 7}) log.record(id);
        System.out.println("get_last(1) = " + log.getLast(1));
        System.out.println("get_last(2) = " + log.getLast(2));
        System.out.println("get_last(3) = " + log.getLast(3));
    }
}
