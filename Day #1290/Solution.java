// Day 1290: Strict point-in-polygon test (ray casting), boundary counts as outside.
// Check edges for on-boundary, then parity of rightward ray crossings. Time O(n), Space O(1).
public class Solution {
    static boolean onSegment(double[] a, double[] b, double[] p) {
        double cross = (b[0] - a[0]) * (p[1] - a[1]) - (b[1] - a[1]) * (p[0] - a[0]);
        if (Math.abs(cross) > 1e-9) return false;
        return p[0] >= Math.min(a[0], b[0]) - 1e-9 && p[0] <= Math.max(a[0], b[0]) + 1e-9 &&
               p[1] >= Math.min(a[1], b[1]) - 1e-9 && p[1] <= Math.max(a[1], b[1]) + 1e-9;
    }

    static boolean inside(double[][] poly, double[] p) {
        int n = poly.length;
        for (int i = 0; i < n; i++)
            if (onSegment(poly[i], poly[(i + 1) % n], p)) return false; // boundary
        boolean res = false;
        for (int i = 0, j = n - 1; i < n; j = i++) {
            double xi = poly[i][0], yi = poly[i][1], xj = poly[j][0], yj = poly[j][1];
            if ((yi > p[1]) != (yj > p[1])) {
                double xint = (xj - xi) * (p[1] - yi) / (yj - yi) + xi;
                if (p[0] < xint) res = !res;
            }
        }
        return res;
    }

    public static void main(String[] args) {
        double[][] square = {{0, 0}, {4, 0}, {4, 4}, {0, 4}};
        System.out.println(inside(square, new double[]{2, 2}) ? "True" : "False"); // True
        System.out.println(inside(square, new double[]{5, 5}) ? "True" : "False"); // False
        System.out.println(inside(square, new double[]{4, 2}) ? "True" : "False"); // False
    }
}
