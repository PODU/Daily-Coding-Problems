// Closest pair of points via divide-and-conquer on x, strip-merge on y.
// Each point tagged with an id for an unambiguous left/right split. Time: O(n log n). Space: O(n).
import java.util.*;

public class Solution {
    static class Pt { long x, y; int id; Pt(long x, long y, int id){this.x=x;this.y=y;this.id=id;} }

    static double dist2(Pt a, Pt b) {
        double dx = a.x - b.x, dy = a.y - b.y;
        return dx*dx + dy*dy;
    }

    static Pt[] rec(Pt[] px, Pt[] py) {
        int n = px.length;
        if (n <= 3) {
            double best = Double.MAX_VALUE; Pt[] bp = {px[0], px[0]};
            for (int i = 0; i < n; i++)
                for (int j = i+1; j < n; j++)
                    if (dist2(px[i], px[j]) < best) { best = dist2(px[i], px[j]); bp = new Pt[]{px[i], px[j]}; }
            return bp;
        }
        int mid = n / 2;
        long midX = px[mid].x;
        Pt[] lx = Arrays.copyOfRange(px, 0, mid);
        Pt[] rx = Arrays.copyOfRange(px, mid, n);
        Set<Integer> leftIds = new HashSet<>();
        for (Pt p : lx) leftIds.add(p.id);
        List<Pt> lyL = new ArrayList<>(), ryL = new ArrayList<>();
        for (Pt p : py) (leftIds.contains(p.id) ? lyL : ryL).add(p);

        Pt[] bl = rec(lx, lyL.toArray(new Pt[0]));
        Pt[] br = rec(rx, ryL.toArray(new Pt[0]));
        Pt[] best = dist2(bl[0], bl[1]) < dist2(br[0], br[1]) ? bl : br;
        double d2 = dist2(best[0], best[1]);

        List<Pt> strip = new ArrayList<>();
        for (Pt p : py) { double dx = p.x - midX; if (dx*dx < d2) strip.add(p); }
        for (int i = 0; i < strip.size(); i++)
            for (int j = i+1; j < strip.size(); j++) {
                double dy = strip.get(j).y - strip.get(i).y;
                if (dy*dy >= d2) break;
                if (dist2(strip.get(i), strip.get(j)) < d2) { d2 = dist2(strip.get(i), strip.get(j)); best = new Pt[]{strip.get(i), strip.get(j)}; }
            }
        return best;
    }

    static Pt[] closestPair(Pt[] pts) {
        Pt[] px = pts.clone(), py = pts.clone();
        Arrays.sort(px, (a,b) -> a.x != b.x ? Long.compare(a.x,b.x) : Long.compare(a.y,b.y));
        Arrays.sort(py, (a,b) -> a.y != b.y ? Long.compare(a.y,b.y) : Long.compare(a.x,b.x));
        return rec(px, py);
    }

    public static void main(String[] args) {
        long[][] raw = {{1,1},{-1,-1},{3,4},{6,1},{-1,-6},{-4,-3}};
        Pt[] pts = new Pt[raw.length];
        for (int i = 0; i < raw.length; i++) pts[i] = new Pt(raw[i][0], raw[i][1], i);
        Pt[] r = closestPair(pts);
        Pt a = r[0], b = r[1];
        if (a.x > b.x || (a.x == b.x && a.y > b.y)) { Pt t = a; a = b; b = t; }
        System.out.println("[(" + a.x + ", " + a.y + "), (" + b.x + ", " + b.y + ")]");
        // [(-1, -1), (1, 1)]
    }
}
