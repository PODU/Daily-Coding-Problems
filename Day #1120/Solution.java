// Day 1120 - Minimum steps to cover points in order (8-directional moves)
// Cost between two points is Chebyshev distance max(|dx|,|dy|); sum them.
// Time: O(n), Space: O(1).
public class Solution {
    static int minSteps(int[][] pts) {
        int total = 0;
        for (int i = 1; i < pts.length; i++)
            total += Math.max(Math.abs(pts[i][0] - pts[i-1][0]),
                              Math.abs(pts[i][1] - pts[i-1][1]));
        return total;
    }

    public static void main(String[] args) {
        int[][] points = {{0, 0}, {1, 1}, {1, 2}};
        System.out.println(minSteps(points)); // 2
    }
}
