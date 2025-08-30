// Day 194: Segments p_i->q_i cross iff order of p and q disagree. Count = inversions of q
// after sorting pairs by p. Merge-sort inversion count. Time O(n log n), Space O(n).
import java.util.*;

public class Solution {
    static long mergeCount(int[] v, int l, int r) {
        if (r - l <= 1) return 0;
        int m = (l + r) / 2;
        long cnt = mergeCount(v, l, m) + mergeCount(v, m, r);
        int[] tmp = new int[r - l];
        int i = l, j = m, k = 0;
        while (i < m && j < r) {
            if (v[i] <= v[j]) tmp[k++] = v[i++];
            else { tmp[k++] = v[j++]; cnt += m - i; }
        }
        while (i < m) tmp[k++] = v[i++];
        while (j < r) tmp[k++] = v[j++];
        System.arraycopy(tmp, 0, v, l, r - l);
        return cnt;
    }

    static long countCrossings(int[] p, int[] q) {
        int n = p.length;
        Integer[] idx = new Integer[n];
        for (int i = 0; i < n; i++) idx[i] = i;
        Arrays.sort(idx, (a, b) -> Integer.compare(p[a], p[b]));
        int[] qs = new int[n];
        for (int k = 0; k < n; k++) qs[k] = q[idx[k]];
        return mergeCount(qs, 0, n);
    }

    public static void main(String[] args) {
        System.out.println(countCrossings(new int[]{1, 2, 3, 4}, new int[]{4, 3, 2, 1}));
    }
}
