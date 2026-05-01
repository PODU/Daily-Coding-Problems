// Day 1451: Next lexicographic permutation in place (wraps to smallest).
// Find rightmost ascent, swap with next-larger suffix element, reverse suffix.
// Time O(n), Space O(1).
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
        for (int l = i + 1, r = n - 1; l < r; l++, r--) {
            int t = a[l]; a[l] = a[r]; a[r] = t;
        }
    }

    static void show(int[] a) {
        nextPermutation(a);
        System.out.println(Arrays.toString(a).replace(" ", ""));
    }

    public static void main(String[] args) {
        show(new int[]{1,2,3}); // [1,3,2]
        show(new int[]{1,3,2}); // [2,1,3]
        show(new int[]{3,2,1}); // [1,2,3]
    }
}
