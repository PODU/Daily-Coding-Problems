// Day 600: Closest pair of points on a plane.
// Approach: divide & conquer with a y-sorted strip check. Time O(n log n), Space O(n).
import java.util.*;

public class Solution {
    static long[][] pts;

    static double dist2(long[] a, long[] b) {
        double dx = a[0] - b[0], dy = a[1] - b[1];
        return dx * dx + dy * dy;
    }

    static double best;
    static long[] bestA, bestB;

    static void consider(long[] a, long[] b) {
        double d = dist2(a, b);
        if (d < best) { best = d; bestA = a; bestB = b; }
    }

    static void rec(long[][] px, int lo, int hi) {
        int n = hi - lo;
        if (n <= 3) {
            for (int i = lo; i < hi; i++)
                for (int j = i + 1; j < hi; j++) consider(px[i], px[j]);
            return;
        }
        int mid = (lo + hi) / 2;
        long midx = px[mid][0];
        rec(px, lo, mid);
        rec(px, mid, hi);
        double dd = Math.sqrt(best);

        List<long[]> strip = new ArrayList<>();
        for (int i = lo; i < hi; i++)
            if (Math.abs((double) px[i][0] - midx) < dd) strip.add(px[i]);
        strip.sort((a, b) -> Long.compare(a[1], b[1]));
        for (int i = 0; i < strip.size(); i++)
            for (int j = i + 1; j < strip.size() && (strip.get(j)[1] - strip.get(i)[1]) < dd; j++) {
                consider(strip.get(i), strip.get(j));
                dd = Math.sqrt(best);
            }
    }

    public static void main(String[] args) {
        pts = new long[][]{{1, 1}, {-1, -1}, {3, 4}, {6, 1}, {-1, -6}, {-4, -3}};
        Arrays.sort(pts, (a, b) -> a[0] != b[0] ? Long.compare(a[0], b[0]) : Long.compare(a[1], b[1]));
        best = Double.MAX_VALUE;
        rec(pts, 0, pts.length);
        long[] a = bestA, b = bestB;
        if (a[0] > b[0] || (a[0] == b[0] && a[1] > b[1])) { long[] t = a; a = b; b = t; }
        System.out.printf("[(%d, %d), (%d, %d)]%n", a[0], a[1], b[0], b[1]);
    }
}
