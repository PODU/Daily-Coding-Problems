// Day 197: Rotate array right by k in-place.
// Triple-reversal: reverse whole, reverse first k, reverse rest. O(n) time, O(1) space.
import java.util.*;

public class Solution {
    static void reverse(int[] a, int i, int j) {
        while (i < j) { int t = a[i]; a[i] = a[j]; a[j] = t; i++; j--; }
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
        int[] a = {1, 2, 3, 4, 5};
        rotateRight(a, 2);
        System.out.println(Arrays.toString(a)); // [4, 5, 1, 2, 3]
    }
}
