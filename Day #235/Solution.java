// Min & Max in ~3N/2 comparisons: process elements in pairs, compare the pair first,
// then smaller vs min and larger vs max. Time: O(N), Space: O(1). Comparisons ~ 3*ceil(N/2)-2.
public class Solution {
    static int[] minMax(int[] a) {
        int n = a.length, mn, mx, i;
        if (n % 2 == 0) {
            if (a[0] < a[1]) { mn = a[0]; mx = a[1]; }
            else { mn = a[1]; mx = a[0]; }
            i = 2;
        } else {
            mn = mx = a[0];
            i = 1;
        }
        while (i < n) {
            int x = a[i], y = a[i + 1];
            if (x < y) {
                if (x < mn) mn = x;
                if (y > mx) mx = y;
            } else {
                if (y < mn) mn = y;
                if (x > mx) mx = x;
            }
            i += 2;
        }
        return new int[]{mn, mx};
    }

    public static void main(String[] args) {
        int[] a = {3, 5, 1, 2, 4, 8, 7};
        int[] r = minMax(a);
        System.out.println("min=" + r[0] + " max=" + r[1]); // min=1 max=8
    }
}
