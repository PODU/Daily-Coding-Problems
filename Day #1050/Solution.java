// Apply permutation where result[P[i]] = array[i] (scatter). In-place cycle-following via swaps:
// O(n) time, O(1) extra space. Also a simple O(n)-space scatter is provided.
import java.util.Arrays;

public class Solution {
    // In-place: follow each cycle with swaps. Each swap settles an element, so <= n swaps total.
    static void applyInPlace(String[] a, int[] P) {
        for (int i = 0; i < a.length; i++) {
            while (P[i] != i) {
                String ts = a[i]; a[i] = a[P[i]]; a[P[i]] = ts;
                int tp = P[i]; P[i] = P[tp]; P[tp] = tp;
            }
        }
    }

    // Simple reference: O(n) time, O(n) space.
    static String[] applySimple(String[] a, int[] P) {
        String[] res = new String[a.length];
        for (int i = 0; i < a.length; i++) res[P[i]] = a[i];
        return res;
    }

    public static void main(String[] args) {
        String[] a = {"a", "b", "c"};
        int[] P = {2, 1, 0}; // result[P[i]] = a[i]
        applyInPlace(a, P);
        System.out.println(Arrays.toString(a));
    }
}
