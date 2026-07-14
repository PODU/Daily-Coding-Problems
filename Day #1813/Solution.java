// Last-N order log via fixed-size circular buffer.
// record: O(1), get_last(i): O(1). Space: O(N).
public class Solution {
    static class OrderLog {
        long[] buf;
        int n, count = 0, head = 0;
        OrderLog(int N) { n = N; buf = new long[N]; }
        void record(long id) {
            buf[head] = id;
            head = (head + 1) % n;
            if (count < n) count++;
        }
        long getLast(int i) {
            int idx = ((head - i) % n + n) % n;
            return buf[idx];
        }
    }

    public static void main(String[] args) {
        OrderLog log = new OrderLog(5);
        long[] ids = {101, 102, 103, 104, 105, 106};
        for (long id : ids) log.record(id);
        System.out.println(log.getLast(1)); // 106
        System.out.println(log.getLast(3)); // 104
    }
}
