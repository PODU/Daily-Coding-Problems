// Closest pair of points via divide & conquer: sort by x, recurse, merge with strip check by y.
// O(n log n) time, O(n) space.
import java.util.*;

public class Solution {
    static long[][] best = new long[2][2];
    static double bestD = Double.MAX_VALUE;

    static double dist(long[] a, long[] b) {
        double dx = a[0]-b[0], dy = a[1]-b[1];
        return Math.sqrt(dx*dx + dy*dy);
    }

    static void consider(long[] a, long[] b) {
        double d = dist(a, b);
        if (d < bestD) { bestD = d; best[0] = a; best[1] = b; }
    }

    // px sorted by x in [lo,hi); on return px[lo,hi) sorted by y
    static double rec(long[][] px, int lo, int hi) {
        int n = hi - lo;
        if (n <= 3) {
            double b = Double.MAX_VALUE;
            for (int i = lo; i < hi; i++)
                for (int j = i+1; j < hi; j++) { consider(px[i], px[j]); b = Math.min(b, dist(px[i], px[j])); }
            Arrays.sort(px, lo, hi, (p,q) -> Long.compare(p[1], q[1]));
            return b;
        }
        int mid = lo + n/2;
        long midx = px[mid][0];
        double dl = rec(px, lo, mid);
        double dr = rec(px, mid, hi);
        double d = Math.min(dl, dr);
        long[][] merged = new long[n][];
        int i = lo, j = mid, k = 0;
        while (i < mid && j < hi) merged[k++] = (px[i][1] <= px[j][1]) ? px[i++] : px[j++];
        while (i < mid) merged[k++] = px[i++];
        while (j < hi) merged[k++] = px[j++];
        for (int t = 0; t < n; t++) px[lo+t] = merged[t];
        List<long[]> strip = new ArrayList<>();
        for (int t = lo; t < hi; t++) if (Math.abs(px[t][0] - midx) < d) strip.add(px[t]);
        for (int a = 0; a < strip.size(); a++)
            for (int b = a+1; b < strip.size() && (strip.get(b)[1] - strip.get(a)[1]) < d; b++) {
                consider(strip.get(a), strip.get(b));
                d = Math.min(d, dist(strip.get(a), strip.get(b)));
            }
        return d;
    }

    public static void main(String[] args) {
        long[][] pts = {{1,1},{-1,-1},{3,4},{6,1},{-1,-6},{-4,-3}};
        Arrays.sort(pts, (a,b) -> a[0] != b[0] ? Long.compare(a[0], b[0]) : Long.compare(a[1], b[1]));
        rec(pts, 0, pts.length);
        long[] a = best[0], b = best[1];
        if (a[0] > b[0] || (a[0] == b[0] && a[1] > b[1])) { long[] t = a; a = b; b = t; }
        System.out.println("[(" + a[0] + ", " + a[1] + "), (" + b[0] + ", " + b[1] + ")]");
    }
}
