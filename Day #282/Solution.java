// Day 282: Detect Pythagorean triplet. Square + sort, then two-pointer per c.
// Time O(N^2), Space O(1) extra (in-place squares).
import java.util.Arrays;

public class Solution {
    static boolean hasTriplet(long[] a) {
        for (int i = 0; i < a.length; i++) a[i] *= a[i];
        Arrays.sort(a);
        int n = a.length;
        for (int c = n - 1; c >= 2; c--) {
            int lo = 0, hi = c - 1;
            while (lo < hi) {
                long s = a[lo] + a[hi];
                if (s == a[c]) return true;
                else if (s < a[c]) lo++;
                else hi--;
            }
        }
        return false;
    }

    public static void main(String[] args) {
        System.out.println(hasTriplet(new long[]{3, 1, 4, 6, 5}));   // true
        System.out.println(hasTriplet(new long[]{10, 4, 6, 12, 5})); // false
    }
}
