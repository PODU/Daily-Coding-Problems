// Day 1680: Strict point-in-polygon. Reject boundary via on-segment test, else
// ray-casting parity. Time O(N), Space O(1).
public class Solution {
    static boolean onSeg(double x1, double y1, double x2, double y2, double px, double py) {
        double cross = (x2 - x1) * (py - y1) - (y2 - y1) * (px - x1);
        if (Math.abs(cross) > 1e-9) return false;
        return px >= Math.min(x1, x2) - 1e-9 && px <= Math.max(x1, x2) + 1e-9 &&
               py >= Math.min(y1, y2) - 1e-9 && py <= Math.max(y1, y2) + 1e-9;
    }

    static boolean inside(double[][] poly, double px, double py) {
        int n = poly.length;
        for (int i = 0; i < n; i++) {
            double[] a = poly[i], b = poly[(i + 1) % n];
            if (onSeg(a[0], a[1], b[0], b[1], px, py)) return false;
        }
        boolean in = false;
        for (int i = 0, j = n - 1; i < n; j = i++) {
            double xi = poly[i][0], yi = poly[i][1], xj = poly[j][0], yj = poly[j][1];
            if ((yi > py) != (yj > py) &&
                px < (xj - xi) * (py - yi) / (yj - yi) + xi)
                in = !in;
        }
        return in;
    }

    public static void main(String[] args) {
        double[][] sq = {{0,0},{4,0},{4,4},{0,4}};
        System.out.println(inside(sq, 2, 2) ? "True" : "False"); // True
        System.out.println(inside(sq, 4, 2) ? "True" : "False"); // False
        System.out.println(inside(sq, 5, 5) ? "True" : "False"); // False
    }
}
