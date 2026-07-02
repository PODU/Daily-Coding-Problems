// Day 1751: Min steps to visit points in order on an 8-directional grid.
// Sum of Chebyshev distances max(|dx|,|dy|) between consecutive points. O(n) time, O(1) space.
public class Solution {
    static int minSteps(int[][] pts) {
        int total = 0;
        for (int i = 1; i < pts.length; i++) {
            int dx = Math.abs(pts[i][0] - pts[i-1][0]);
            int dy = Math.abs(pts[i][1] - pts[i-1][1]);
            total += Math.max(dx, dy);
        }
        return total;
    }

    public static void main(String[] args) {
        int[][] pts = {{0,0},{1,1},{1,2}};
        System.out.println(minSteps(pts)); // 2
    }
}
