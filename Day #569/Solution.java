// Find min and max using ~3*ceil(N/2) comparisons (pairwise method).
// Time: O(N) with <2N comparisons. Space: O(1).
public class Solution {
    static long cmps = 0;

    static int[] minMax(int[] a) {
        int n = a.length, mn, mx, i;
        if ((n & 1) == 1) { mn = mx = a[0]; i = 1; }   // odd: seed with first
        else {                                          // even: seed with first pair
            cmps++;
            if (a[0] < a[1]) { mn = a[0]; mx = a[1]; }
            else { mn = a[1]; mx = a[0]; }
            i = 2;
        }
        for (; i + 1 < n; i += 2) {
            int lo, hi;
            cmps++;
            if (a[i] < a[i + 1]) { lo = a[i]; hi = a[i + 1]; }
            else { lo = a[i + 1]; hi = a[i]; }
            cmps++; if (lo < mn) mn = lo;
            cmps++; if (hi > mx) mx = hi;
        }
        return new int[]{mn, mx};
    }

    public static void main(String[] args) {
        int[] a = {7, 2, 9, 4, 1, 8, 3};
        int[] r = minMax(a);
        System.out.println("min=" + r[0] + " max=" + r[1]);
    }
}
