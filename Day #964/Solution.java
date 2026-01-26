// Day 964: Order log keeping last N ids with O(1) record/get_last.
// Approach: fixed-size circular buffer. Time O(1) per op, Space O(N).
public class Solution {
    static class OrderLog {
        int[] buf;
        int n, count = 0;
        OrderLog(int N) { buf = new int[N]; n = N; }
        void record(int orderId) { buf[count % n] = orderId; count++; }
        int getLast(int i) { return buf[((count - i) % n + n) % n]; }
    }

    public static void main(String[] args) {
        OrderLog log = new OrderLog(3);
        log.record(10);
        log.record(20);
        log.record(30);
        log.record(40); // evicts 10
        System.out.println(log.getLast(1)); // 40
        System.out.println(log.getLast(2)); // 30
        System.out.println(log.getLast(3)); // 20
    }
}
