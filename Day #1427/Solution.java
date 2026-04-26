// Day 1427: Rotate array right by k in-place.
// Approach: triple reversal (reverse all, reverse first k, reverse rest).
// Time: O(n), Space: O(1).
import java.util.Arrays;

public class Solution {
    static void reverse(int[] a, int i, int j) {
        while (i < j) {
            int t = a[i]; a[i] = a[j]; a[j] = t;
            i++; j--;
        }
    }

    static void rotate(int[] a, int k) {
        int n = a.length;
        if (n == 0) return;
        k %= n;
        reverse(a, 0, n - 1);
        reverse(a, 0, k - 1);
        reverse(a, k, n - 1);
    }

    public static void main(String[] args) {
        int[] a = {1, 2, 3, 4, 5, 6, 7};
        rotate(a, 3);
        System.out.println(Arrays.toString(a)); // [5, 6, 7, 1, 2, 3, 4]
    }
}
