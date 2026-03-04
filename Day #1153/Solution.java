// Day 1153: Min steps to visit points in order (8-directional moves).
// Sum of Chebyshev distances max(|dx|,|dy|) between consecutive points. O(n) time, O(1) space.
public class Solution {
    static int minSteps(int[][] pts) {
        int total = 0;
        for (int i = 1; i < pts.length; i++)
            total += Math.max(Math.abs(pts[i][0] - pts[i-1][0]), Math.abs(pts[i][1] - pts[i-1][1]));
        return total;
    }

    public static void main(String[] args) {
        int[][] pts = {{0, 0}, {1, 1}, {1, 2}};
        System.out.println(minSteps(pts)); // 2
    }
}
