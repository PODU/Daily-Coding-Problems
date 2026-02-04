// Rectangle intersection area: O(1) time, O(1) space.
// top_left is top y, height extends downward. x_overlap*y_overlap clamped at 0.
public class Solution {
    static double intersectArea(double[] a, double[] b) {
        // rect = {left, top, width, height}
        double aRight = a[0] + a[2], bRight = b[0] + b[2];
        double aBottom = a[1] - a[3], bBottom = b[1] - b[3];
        double xo = Math.max(0, Math.min(aRight, bRight) - Math.max(a[0], b[0]));
        double yo = Math.max(0, Math.min(a[1], b[1]) - Math.max(aBottom, bBottom));
        return xo * yo;
    }

    public static void main(String[] args) {
        double[] r1 = {1, 4, 3, 3};
        double[] r2 = {0, 5, 4, 3};
        double area = intersectArea(r1, r2);
        if (area == Math.floor(area)) System.out.println((long) area);
        else System.out.println(area);
    }
}
