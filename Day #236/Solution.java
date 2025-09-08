// Point in polygon: ray-casting (even-odd rule). Boundary points are detected separately
// and return false. Time: O(N), Space: O(1).
public class Solution {
    static boolean onSegment(double px, double py, double ax, double ay, double bx, double by) {
        double cross = (bx - ax) * (py - ay) - (by - ay) * (px - ax);
        if (Math.abs(cross) > 1e-9) return false;
        return Math.min(ax, bx) - 1e-9 <= px && px <= Math.max(ax, bx) + 1e-9 &&
               Math.min(ay, by) - 1e-9 <= py && py <= Math.max(ay, by) + 1e-9;
    }

    static boolean inside(double[][] poly, double px, double py) {
        int n = poly.length;
        boolean in = false;
        for (int i = 0, j = n - 1; i < n; j = i++) {
            double xi = poly[i][0], yi = poly[i][1];
            double xj = poly[j][0], yj = poly[j][1];
            if (onSegment(px, py, xi, yi, xj, yj)) return false; // boundary
            boolean intersect = ((yi > py) != (yj > py)) &&
                (px < (xj - xi) * (py - yi) / (yj - yi) + xi);
            if (intersect) in = !in;
        }
        return in;
    }

    public static void main(String[] args) {
        double[][] poly = {{0, 0}, {4, 0}, {4, 4}, {0, 4}};
        System.out.println(inside(poly, 2, 2)); // true
        System.out.println(inside(poly, 4, 2)); // false (boundary)
        System.out.println(inside(poly, 5, 5)); // false (outside)
    }
}
