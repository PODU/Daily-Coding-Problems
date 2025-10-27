// Day 505: Rotate array right by k in-place via three reversals.
// Time O(n), Space O(1).
import java.util.Arrays;

public class Solution {
    static void reverse(int[] a, int lo, int hi) {
        while (lo < hi) {
            int t = a[lo];
            a[lo] = a[hi];
            a[hi] = t;
            lo++;
            hi--;
        }
    }

    static void rotateRight(int[] a, int k) {
        int n = a.length;
        if (n == 0) return;
        k %= n;
        reverse(a, 0, n - 1);
        reverse(a, 0, k - 1);
        reverse(a, k, n - 1);
    }

    public static void main(String[] args) {
        int[] a = {1, 2, 3, 4, 5, 6, 7};
        rotateRight(a, 3);
        System.out.println(Arrays.toString(a)); // [5, 6, 7, 1, 2, 3, 4]
    }
}
