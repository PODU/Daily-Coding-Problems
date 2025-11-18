// Brick wall: hashmap of prefix-sum edge positions (excluding full-width edge).
// Answer = numRows - maxEdgeCount. Time O(total bricks), Space O(distinct edges).
import java.util.*;

public class Solution {
    static int leastBricks(int[][] wall) {
        Map<Long, Integer> edges = new HashMap<>();
        int maxCount = 0;
        for (int[] row : wall) {
            long sum = 0;
            for (int i = 0; i + 1 < row.length; i++) {
                sum += row[i];
                int c = edges.merge(sum, 1, Integer::sum);
                maxCount = Math.max(maxCount, c);
            }
        }
        return wall.length - maxCount;
    }

    public static void main(String[] args) {
        int[][] wall = {{3,5,1,1},{2,3,3,2},{5,5},{4,4,2},{1,3,3,3},{1,1,6,1,1}};
        System.out.println(leastBricks(wall));
    }
}
