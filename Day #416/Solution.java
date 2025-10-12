// Day 416: Min king-moves to visit points in order = sum of Chebyshev distances max(|dx|,|dy|).
// Time O(n), Space O(1).
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
        System.out.println(minSteps(pts));
    }
}
