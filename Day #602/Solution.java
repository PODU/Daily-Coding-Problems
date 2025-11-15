// Approach: sort segments by p, then count inversions in the q-order via merge sort.
// Two segments cross iff their p-order and q-order disagree => an inversion. Time O(n log n), Space O(n).
import java.util.*;

public class Solution {
    static long mergeCount(int[] a, int l, int r) {
        if (r - l <= 1) return 0;
        int m = (l + r) / 2;
        long inv = mergeCount(a, l, m) + mergeCount(a, m, r);
        int[] tmp = new int[r - l];
        int i = l, j = m, k = 0;
        while (i < m && j < r) {
            if (a[i] <= a[j]) tmp[k++] = a[i++];
            else { tmp[k++] = a[j++]; inv += m - i; }
        }
        while (i < m) tmp[k++] = a[i++];
        while (j < r) tmp[k++] = a[j++];
        System.arraycopy(tmp, 0, a, l, tmp.length);
        return inv;
    }

    static long countIntersections(int[] p, int[] q) {
        int n = p.length;
        Integer[] idx = new Integer[n];
        for (int i = 0; i < n; i++) idx[i] = i;
        Arrays.sort(idx, (x, y) -> Integer.compare(p[x], p[y]));
        int[] qs = new int[n];
        for (int i = 0; i < n; i++) qs[i] = q[idx[i]];
        return mergeCount(qs, 0, n);
    }

    public static void main(String[] args) {
        int[] p = {1, 2, 3, 4};
        int[] q = {4, 3, 2, 1};
        System.out.println(countIntersections(p, q));
    }
}
