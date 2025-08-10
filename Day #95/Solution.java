// Day 95: Next lexicographic permutation in place. Find rightmost ascent, swap
// with next-larger suffix element, reverse suffix. O(n) time, O(1) space.
import java.util.*;

public class Solution {
    static void nextPermutation(int[] a) {
        int n = a.length, i = n - 2;
        while (i >= 0 && a[i] >= a[i + 1]) i--;
        if (i >= 0) {
            int j = n - 1;
            while (a[j] <= a[i]) j--;
            int t = a[i]; a[i] = a[j]; a[j] = t;
        }
        for (int lo = i + 1, hi = n - 1; lo < hi; lo++, hi--) {
            int t = a[lo]; a[lo] = a[hi]; a[hi] = t;
        }
    }

    public static void main(String[] args) {
        int[][] tests = {{1, 2, 3}, {1, 3, 2}, {3, 2, 1}};
        for (int[] a : tests) {
            nextPermutation(a);
            System.out.println(Arrays.toString(a));
        }
        // [1, 3, 2] / [2, 1, 3] / [1, 2, 3]
    }
}
