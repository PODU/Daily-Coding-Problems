// Fenwick/BIT over 24 hours: point update, prefix-sum range query.
// update O(log n), query O(log n).
public class Solution {
    static class BIT {
        int n;
        long[] tree;
        BIT(int n) {
            this.n = n;
            tree = new long[n + 1];
        }
        void update(int hour, long value) {
            for (int i = hour + 1; i <= n; i += i & (-i)) tree[i] += value;
        }
        long prefix(int idx) { // sum of [0..idx]
            long s = 0;
            for (int i = idx + 1; i > 0; i -= i & (-i)) s += tree[i];
            return s;
        }
        long query(int start, int end) { // inclusive
            return prefix(end) - (start > 0 ? prefix(start - 1) : 0);
        }
    }

    public static void main(String[] args) {
        BIT bit = new BIT(24);
        bit.update(0, 5);
        bit.update(3, 10);
        bit.update(23, 2);
        bit.update(3, 1);
        System.out.println("query(0, 3) = " + bit.query(0, 3));
        System.out.println("query(0, 23) = " + bit.query(0, 23));
        System.out.println("query(4, 23) = " + bit.query(4, 23));
        System.out.println("query(3, 3) = " + bit.query(3, 3));
    }
}
