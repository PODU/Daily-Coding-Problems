// Day 1295: Fixed-size log of last N order ids via circular buffer.
// record and getLast are both O(1) time, O(N) space.
public class Solution {
    static class OrderLog {
        long[] buf;
        int n, head = 0, count = 0;

        OrderLog(int N) { buf = new long[N]; n = N; }

        void record(long id) {
            buf[head] = id;
            head = (head + 1) % n;
            if (count < n) count++;
        }

        long getLast(int i) { // 1 = most recent
            int idx = ((head - i) % n + n) % n;
            return buf[idx];
        }
    }

    public static void main(String[] args) {
        OrderLog log = new OrderLog(3);
        log.record(10);
        log.record(20);
        log.record(30);
        System.out.println(log.getLast(1)); // 30
        System.out.println(log.getLast(2)); // 20
        log.record(40);
        System.out.println(log.getLast(1)); // 40
        System.out.println(log.getLast(3)); // 20
    }
}
