// Count intersecting segment pairs: sort segments by p, then count inversions in q.
// Two segments cross iff their p-order and q-order disagree. O(n log n) time, O(n) space.
import java.util.*;

public class Solution {
    static long mergeCount(int[] a, int l, int r) {
        if (r - l <= 1) return 0;
        int m = (l + r) / 2;
        long c = mergeCount(a, l, m) + mergeCount(a, m, r);
        int[] tmp = new int[r - l];
        int i = l, j = m, t = 0;
        while (i < m && j < r) {
            if (a[i] <= a[j]) tmp[t++] = a[i++];
            else { tmp[t++] = a[j++]; c += m - i; }
        }
        while (i < m) tmp[t++] = a[i++];
        while (j < r) tmp[t++] = a[j++];
        System.arraycopy(tmp, 0, a, l, r - l);
        return c;
    }

    static long countIntersections(int[] p, int[] q) {
        int n = p.length;
        Integer[] idx = new Integer[n];
        for (int k = 0; k < n; k++) idx[k] = k;
        Arrays.sort(idx, (x, y) -> Integer.compare(p[x], p[y]));
        int[] qq = new int[n];
        for (int k = 0; k < n; k++) qq[k] = q[idx[k]];
        return mergeCount(qq, 0, n);
    }

    public static void main(String[] args) {
        int[] p = {1, 2, 3}, q = {3, 1, 2};
        System.out.println(countIntersections(p, q)); // 2
    }
}
