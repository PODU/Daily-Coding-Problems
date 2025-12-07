// Day 711: Point strictly inside polygon. First reject boundary via on-segment
// test, then ray-casting parity test. Time O(N) per query.
public class Solution {
    static class P { double x, y; P(double x, double y){this.x=x;this.y=y;} }

    static boolean onSeg(P a, P b, P p) {
        double cross = (b.x - a.x) * (p.y - a.y) - (b.y - a.y) * (p.x - a.x);
        if (Math.abs(cross) > 1e-9) return false;
        return Math.min(a.x, b.x) - 1e-9 <= p.x && p.x <= Math.max(a.x, b.x) + 1e-9 &&
               Math.min(a.y, b.y) - 1e-9 <= p.y && p.y <= Math.max(a.y, b.y) + 1e-9;
    }

    static boolean inside(P[] poly, P p) {
        int n = poly.length;
        for (int i = 0; i < n; i++)
            if (onSeg(poly[i], poly[(i + 1) % n], p)) return false;
        boolean in = false;
        for (int i = 0, j = n - 1; i < n; j = i++) {
            if ((poly[i].y > p.y) != (poly[j].y > p.y)) {
                double xint = (poly[j].x - poly[i].x) * (p.y - poly[i].y) /
                              (poly[j].y - poly[i].y) + poly[i].x;
                if (p.x < xint) in = !in;
            }
        }
        return in;
    }

    public static void main(String[] args) {
        P[] sq = {new P(0,0), new P(4,0), new P(4,4), new P(0,4)};
        System.out.println(inside(sq, new P(2,2)) ? "True" : "False");
        System.out.println(inside(sq, new P(4,2)) ? "True" : "False");
        System.out.println(inside(sq, new P(5,5)) ? "True" : "False");
    }
}
