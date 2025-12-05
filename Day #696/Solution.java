// Day 696: 24-hour subscriber array with point update + inclusive range-sum query.
// Approach: Fenwick (Binary Indexed) Tree. update O(log n), query O(log n), space O(n).
public class Solution {
    static class Fenwick {
        long[] t; int n;
        Fenwick(int n) { this.n = n; t = new long[n + 1]; }
        void add(int i, long v) { for (i++; i <= n; i += i & -i) t[i] += v; }
        long pref(int i) { long s = 0; for (i++; i > 0; i -= i & -i) s += t[i]; return s; }
        long range(int l, int r) { return pref(r) - (l > 0 ? pref(l - 1) : 0); }
    }

    static class Subscribers {
        Fenwick f = new Fenwick(24);
        void update(int hour, int value) { f.add(hour, value); }
        long query(int start, int end) { return f.range(start, end); }
    }

    public static void main(String[] args) {
        Subscribers s = new Subscribers();
        s.update(3, 10); s.update(5, 7); s.update(10, 4);
        System.out.println(s.query(3, 10)); // 21
        System.out.println(s.query(0, 4));  // 10
        s.update(3, 5);
        System.out.println(s.query(3, 10)); // 26
    }
}
