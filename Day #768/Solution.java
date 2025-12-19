// Day 768: Find min and max together using ~3N/2 comparisons (< 2*(N-2)).
// Process elements in pairs: compare the pair, then smaller vs min, larger vs max.
public class Solution {
    static int[] minMax(int[] a) {
        int n = a.length, mn, mx, i;
        if (n % 2 == 0) { mn = Math.min(a[0], a[1]); mx = Math.max(a[0], a[1]); i = 2; }
        else { mn = mx = a[0]; i = 1; }
        for (; i < n; i += 2) {
            int lo = a[i], hi = a[i + 1];
            if (lo > hi) { int t = lo; lo = hi; hi = t; }
            mn = Math.min(mn, lo);
            mx = Math.max(mx, hi);
        }
        return new int[]{mn, mx};
    }

    public static void main(String[] args) {
        int[] r = minMax(new int[]{3, 5, 1, 2, 4, 8, 7});
        System.out.println("min=" + r[0] + " max=" + r[1]); // min=1 max=8
    }
}
