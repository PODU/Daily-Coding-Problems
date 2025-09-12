// Day 260: Reconstruct a permutation of [0..N] consistent with +/- sign array.
// Grow a list: on '+' append max+1, on '-' append min-1; shift by -min into [0..N].
// O(n) time, O(n) space.
import java.util.*;

public class Solution {
    static int[] reconstruct(int[] signs) { // signs[0] is sentinel (None)
        int n = signs.length;
        int[] res = new int[n];
        int curMax = 0, curMin = 0;
        res[0] = 0;
        for (int i = 1; i < n; i++) {
            if (signs[i] > 0) res[i] = ++curMax;
            else              res[i] = --curMin;
        }
        int off = -curMin;
        for (int i = 0; i < n; i++) res[i] += off;
        return res;
    }

    public static void main(String[] args) {
        int[] signs = {0, 1, 1, -1, 1}; // [None, +, +, -, +]
        int[] r = reconstruct(signs);
        StringBuilder sb = new StringBuilder("[");
        for (int i = 0; i < r.length; i++) { if (i > 0) sb.append(", "); sb.append(r[i]); }
        sb.append("]");
        System.out.println(sb);
    }
}
