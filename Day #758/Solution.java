// Day 758: Rotate a list left by k in place using the 3-reversal trick.
// No copy; ~n swaps total (each reversal swaps floor(len/2) pairs).
// Time: O(n), Space: O(1).
import java.util.*;

public class Solution {
    static int reverseRange(int[] a, int i, int j) {
        int swaps = 0;
        while (i < j) {
            int t = a[i]; a[i] = a[j]; a[j] = t;
            i++; j--; swaps++;
        }
        return swaps;
    }

    static int rotateLeft(int[] a, int k) {
        int n = a.length;
        if (n == 0) return 0;
        k %= n;
        int swaps = 0;
        swaps += reverseRange(a, 0, k - 1);
        swaps += reverseRange(a, k, n - 1);
        swaps += reverseRange(a, 0, n - 1);
        return swaps;
    }

    public static void main(String[] args) {
        int[] a = {1, 2, 3, 4, 5, 6};
        int swaps = rotateLeft(a, 2);
        System.out.println(Arrays.toString(a));  // [3, 4, 5, 6, 1, 2]
        System.out.println("swaps: " + swaps);
    }
}
