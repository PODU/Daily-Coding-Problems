// Day 126: Rotate list left by k in-place via 3 reversals.
// O(n) time, O(1) extra space, ~n swaps total.
import java.util.*;

public class Solution {
    static void reverse(int[] a, int i, int j) {
        while (i < j) {
            int t = a[i]; a[i] = a[j]; a[j] = t;
            i++; j--;
        }
    }

    static void rotateLeft(int[] a, int k) {
        int n = a.length;
        if (n == 0) return;
        k %= n;
        reverse(a, 0, k - 1);
        reverse(a, k, n - 1);
        reverse(a, 0, n - 1);
    }

    public static void main(String[] args) {
        int[] a = {1, 2, 3, 4, 5, 6};
        rotateLeft(a, 2);
        System.out.println(Arrays.toString(a));
    }
}
