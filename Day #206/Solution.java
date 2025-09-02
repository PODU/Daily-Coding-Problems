// Day 206: Apply permutation P to array (result[P[i]] = a[i]).
// In-place via cycle following on the permutation. Time: O(n), Space: O(1).
import java.util.*;

public class Solution {
    static void applyPermutation(String[] a, int[] p) {
        for (int i = 0; i < a.length; i++) {
            while (p[i] != i) {
                int j = p[i];
                String t = a[i]; a[i] = a[j]; a[j] = t;
                p[i] = p[j]; p[j] = j;
            }
        }
    }

    public static void main(String[] args) {
        String[] a = {"a", "b", "c"};
        applyPermutation(a, new int[]{2, 1, 0});
        System.out.println(Arrays.toString(a)); // [c, b, a]
    }
}
