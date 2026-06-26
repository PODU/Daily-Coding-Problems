// Day 1718: Count intersecting segment pairs (p_i on y=0 -> q_i on y=1).
// Two segments cross iff their (p, q) ordering is inverted: sort by p,
// count inversions in q via merge sort. Time: O(n log n), Space: O(n).
import java.util.*;

public class Solution {
    static long mergeCount(int[] a, int l, int r) {
        if (r - l <= 1) return 0;
        int mid = (l + r) / 2;
        long inv = mergeCount(a, l, mid) + mergeCount(a, mid, r);
        int[] tmp = new int[r - l];
        int i = l, j = mid, t = 0;
        while (i < mid && j < r) {
            if (a[i] <= a[j]) tmp[t++] = a[i++];
            else { inv += mid - i; tmp[t++] = a[j++]; }
        }
        while (i < mid) tmp[t++] = a[i++];
        while (j < r) tmp[t++] = a[j++];
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
        int[] q = {2, 1, 4, 3};
        System.out.println("Intersecting pairs: " + countIntersections(p, q));
    }
}
