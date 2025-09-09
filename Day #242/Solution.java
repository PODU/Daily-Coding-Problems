// Fenwick/Binary Indexed Tree over 24 hours.
// update: O(log n), query (prefix-diff): O(log n). Space O(n).
public class Solution {
    static class BIT {
        int n;
        long[] tree;
        BIT(int n) { this.n = n; tree = new long[n + 1]; }
        void add(int i, long v) {           // 0-based index
            for (i++; i <= n; i += i & (-i)) tree[i] += v;
        }
        long prefix(int i) {                // sum of [0..i], 0-based
            long s = 0;
            for (i++; i > 0; i -= i & (-i)) s += tree[i];
            return s;
        }
        long query(int l, int r) {          // inclusive [l..r]
            return prefix(r) - (l > 0 ? prefix(l - 1) : 0);
        }
        void update(int hour, long value) { add(hour, value); }
    }

    public static void main(String[] args) {
        BIT bit = new BIT(24);
        bit.update(2, 5);
        bit.update(5, 3);
        bit.update(23, 10);
        System.out.println("query(2,5) = " + bit.query(2, 5));
        System.out.println("query(0,23) = " + bit.query(0, 23));
    }
}
