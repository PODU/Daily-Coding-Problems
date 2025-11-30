// Day 673: K nearest points to a center. Sort by squared distance (max-heap of size k also works).
// Time O(n log n) here, Space O(n). No sqrt needed for comparison.
import java.util.*;

public class Solution {
    static int[][] kNearest(int[][] pts, int[] c, int k) {
        Integer[] idx = new Integer[pts.length];
        for (int i = 0; i < idx.length; i++) idx[i] = i;
        Arrays.sort(idx, (a, b) -> {
            long da = d2(pts[a], c), db = d2(pts[b], c);
            return da != db ? Long.compare(da, db) : Integer.compare(a, b);
        });
        int[][] res = new int[Math.min(k, pts.length)][];
        for (int i = 0; i < res.length; i++) res[i] = pts[idx[i]];
        return res;
    }

    static long d2(int[] p, int[] c) {
        long dx = p[0] - c[0], dy = p[1] - c[1];
        return dx * dx + dy * dy;
    }

    public static void main(String[] args) {
        int[][] pts = {{0, 0}, {5, 4}, {3, 1}};
        int[][] r = kNearest(pts, new int[]{1, 2}, 2);
        StringBuilder sb = new StringBuilder("[");
        for (int i = 0; i < r.length; i++)
            sb.append("(").append(r[i][0]).append(", ").append(r[i][1]).append(")")
              .append(i + 1 < r.length ? ", " : "");
        System.out.println(sb.append("]")); // [(0, 0), (3, 1)]
    }
}
