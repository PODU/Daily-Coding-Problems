// Approach: Circular (ring) buffer of size N. record/get_last are O(1); O(N) space.
public class Solution {
    static class OrderLog {
        int[] buf;
        int n, count = 0, head = 0; // head = next write position
        OrderLog(int N) { buf = new int[N]; n = N; }
        void record(int orderId) {
            buf[head] = orderId;
            head = (head + 1) % n;
            if (count < n) count++;
        }
        // i is 1-based: get_last(1) is the most recent
        int getLast(int i) {
            int idx = ((head - i) % n + n) % n;
            return buf[idx];
        }
    }

    public static void main(String[] args) {
        OrderLog log = new OrderLog(3);
        for (int x : new int[]{1, 2, 3, 4, 5}) log.record(x);
        System.out.println("get_last(1) = " + log.getLast(1));
        System.out.println("get_last(2) = " + log.getLast(2));
        System.out.println("get_last(3) = " + log.getLast(3));
    }
}
