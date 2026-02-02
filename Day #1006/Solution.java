// Count intersecting segment pairs: sort by p, count inversions in q via merge sort.
// Time O(n log n), Space O(n).
import java.util.*;

public class Solution {
    static long mergeCount(int[] a, int[] tmp, int l, int r) {
        if (r - l <= 1) return 0;
        int m = (l + r) / 2;
        long cnt = mergeCount(a, tmp, l, m) + mergeCount(a, tmp, m, r);
        int i = l, j = m, k = l;
        while (i < m && j < r) {
            if (a[i] <= a[j]) tmp[k++] = a[i++];
            else { tmp[k++] = a[j++]; cnt += m - i; }
        }
        while (i < m) tmp[k++] = a[i++];
        while (j < r) tmp[k++] = a[j++];
        for (int t = l; t < r; t++) a[t] = tmp[t];
        return cnt;
    }

    static long countIntersections(int[] p, int[] q) {
        int n = p.length;
        Integer[] idx = new Integer[n];
        for (int i = 0; i < n; i++) idx[i] = i;
        Arrays.sort(idx, (x, y) -> Integer.compare(p[x], p[y]));
        int[] qs = new int[n];
        for (int i = 0; i < n; i++) qs[i] = q[idx[i]];
        return mergeCount(qs, new int[n], 0, n);
    }

    public static void main(String[] args) {
        int[] p = {1, 2, 3, 4};
        int[] q = {4, 3, 2, 1};
        System.out.println(countIntersections(p, q)); // 6
    }
}
