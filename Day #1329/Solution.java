// Day 1329: Closest pair of points via divide & conquer. O(n log n) time.
// Sort by x, recurse on halves, then check the middle strip ordered by y (each point vs next ~7).
import java.util.*;

public class Solution {
    static long[][] best = new long[2][2];
    static double bestD;

    static double dist(long[] a, long[] b) {
        double dx = a[0] - b[0], dy = a[1] - b[1];
        return Math.sqrt(dx * dx + dy * dy);
    }

    static void consider(long[] a, long[] b) {
        double d = dist(a, b);
        if (d < bestD) { bestD = d; best[0] = a; best[1] = b; }
    }

    static double rec(long[][] px, long[][] py) {
        int n = px.length;
        if (n <= 3) {
            double b = 1e18;
            for (int i = 0; i < n; i++)
                for (int j = i + 1; j < n; j++) {
                    double d = dist(px[i], px[j]);
                    if (d < b) b = d;
                    consider(px[i], px[j]);
                }
            return b;
        }
        int mid = n / 2;
        long[] pivot = px[mid];
        long midX = pivot[0];
        long[][] lx = Arrays.copyOfRange(px, 0, mid), rx = Arrays.copyOfRange(px, mid, n);
        List<long[]> lyL = new ArrayList<>(), ryL = new ArrayList<>();
        for (long[] p : py) {
            if (p[0] < pivot[0] || (p[0] == pivot[0] && p[1] < pivot[1])) lyL.add(p);
            else ryL.add(p);
        }
        long[][] ly = lyL.toArray(new long[0][]), ry = ryL.toArray(new long[0][]);

        double dl = rec(lx, ly), dr = rec(rx, ry);
        double d = Math.min(dl, dr);

        List<long[]> strip = new ArrayList<>();
        for (long[] p : py) if (Math.abs(p[0] - midX) < d) strip.add(p);
        for (int i = 0; i < strip.size(); i++)
            for (int j = i + 1; j < strip.size() && (strip.get(j)[1] - strip.get(i)[1]) < d; j++) {
                double cur = dist(strip.get(i), strip.get(j));
                if (cur < d) d = cur;
                consider(strip.get(i), strip.get(j));
            }
        return d;
    }

    public static void main(String[] args) {
        long[][] pts = {{1,1},{-1,-1},{3,4},{6,1},{-1,-6},{-4,-3}};
        long[][] px = pts.clone(), py = pts.clone();
        Arrays.sort(px, (a, b) -> a[0] != b[0] ? Long.compare(a[0], b[0]) : Long.compare(a[1], b[1]));
        Arrays.sort(py, (a, b) -> Long.compare(a[1], b[1]));
        bestD = 1e18;
        rec(px, py);
        long[] a = best[0], b = best[1];
        if (a[0] > b[0] || (a[0] == b[0] && a[1] > b[1])) { long[] t = a; a = b; b = t; }
        System.out.printf("[(%d, %d), (%d, %d)]%n", a[0], a[1], b[0], b[1]);
        // [(-1, -1), (1, 1)]
    }
}
