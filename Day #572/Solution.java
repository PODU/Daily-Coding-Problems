// Next greater permutation in-place (lexicographic). If none, wrap to smallest.
// Approach: find pivot, successor swap, reverse suffix. Time O(n), Space O(1).
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

    static void run(int[] a) {
        nextPermutation(a);
        System.out.println(Arrays.toString(a).replace(" ", ""));
    }

    public static void main(String[] args) {
        run(new int[]{1, 2, 3});
        run(new int[]{1, 3, 2});
        run(new int[]{3, 2, 1});
    }
}
