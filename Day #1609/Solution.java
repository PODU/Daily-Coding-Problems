// Subscribers-per-hour over 24 hours via Fenwick/BIT. update(hour,val)+=, query(start,end)=inclusive range sum.
// Time O(log n) per op, Space O(n).
public class Solution {
    static class Fenwick {
        int n;
        long[] tree;
        Fenwick(int n) { this.n = n; tree = new long[n + 1]; }
        void update(int i, long v) { for (i++; i <= n; i += i & -i) tree[i] += v; }
        long pref(int i) { long s = 0; for (i++; i > 0; i -= i & -i) s += tree[i]; return s; }
        long query(int l, int r) { return pref(r) - (l > 0 ? pref(l - 1) : 0); }
    }

    public static void main(String[] args) {
        Fenwick bit = new Fenwick(24); // all zeros
        bit.update(0, 5);
        bit.update(3, 10);
        bit.update(23, 2);
        System.out.println(bit.query(0, 23)); // 17
        System.out.println(bit.query(0, 3));  // 15
        System.out.println(bit.query(1, 2));  // 0
    }
}
