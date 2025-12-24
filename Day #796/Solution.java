// Day 796: Point strictly inside a polygon.
// Ray-casting (even-odd rule) + on-boundary check. Time O(N), Space O(1).
public class Solution {
    static double[][] poly;

    static boolean onSegment(double[] a, double[] b, double px, double py) {
        double cross = (b[0] - a[0]) * (py - a[1]) - (b[1] - a[1]) * (px - a[0]);
        if (Math.abs(cross) > 1e-9) return false;
        return Math.min(a[0], b[0]) - 1e-9 <= px && px <= Math.max(a[0], b[0]) + 1e-9 &&
               Math.min(a[1], b[1]) - 1e-9 <= py && py <= Math.max(a[1], b[1]) + 1e-9;
    }

    static boolean inside(double[][] poly, double px, double py) {
        int n = poly.length;
        for (int i = 0; i < n; i++)
            if (onSegment(poly[i], poly[(i + 1) % n], px, py)) return false;
        boolean in = false;
        for (int i = 0, j = n - 1; i < n; j = i++) {
            if (((poly[i][1] > py) != (poly[j][1] > py)) &&
                (px < (poly[j][0] - poly[i][0]) * (py - poly[i][1]) /
                          (poly[j][1] - poly[i][1]) + poly[i][0]))
                in = !in;
        }
        return in;
    }

    public static void main(String[] args) {
        double[][] square = {{0, 0}, {4, 0}, {4, 4}, {0, 4}};
        System.out.println(inside(square, 2, 2)); // true
        System.out.println(inside(square, 4, 2)); // false (boundary)
        System.out.println(inside(square, 5, 5)); // false
    }
}
