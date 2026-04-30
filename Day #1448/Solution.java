// Day 1448: Fewest bricks cut by a vertical line through a brick wall.
// Count internal edge positions via prefix sums; answer = rows - maxEdgeCount.
// Time O(total bricks), Space O(distinct edges).
import java.util.*;

public class Solution {
    static int leastBricks(int[][] wall) {
        Map<Long, Integer> edges = new HashMap<>();
        int best = 0;
        for (int[] row : wall) {
            long pos = 0;
            for (int i = 0; i + 1 < row.length; i++) { // skip far right edge
                pos += row[i];
                int c = edges.merge(pos, 1, Integer::sum);
                best = Math.max(best, c);
            }
        }
        return wall.length - best;
    }

    public static void main(String[] args) {
        int[][] wall = {
            {3,5,1,1},{2,3,3,2},{5,5},{4,4,2},{1,3,3,3},{1,1,6,1,1}
        };
        System.out.println(leastBricks(wall)); // 2
    }
}
