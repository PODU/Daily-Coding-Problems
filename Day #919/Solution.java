// Brick wall: count edge offsets (cumulative sums excluding last) via hashmap.
// Fewest cuts = numRows - maxAlignedEdges. Time O(total bricks), Space O(distinct edges).
import java.util.*;

public class Solution {
    static int leastBricks(int[][] wall) {
        Map<Long, Integer> freq = new HashMap<>();
        int best = 0;
        for (int[] row : wall) {
            long sum = 0;
            for (int i = 0; i + 1 < row.length; i++) {
                sum += row[i];
                int c = freq.merge(sum, 1, Integer::sum);
                best = Math.max(best, c);
            }
        }
        return wall.length - best;
    }

    public static void main(String[] args) {
        int[][] wall = {{3,5,1,1},{2,3,3,2},{5,5},{4,4,2},{1,3,3,3},{1,1,6,1,1}};
        System.out.println(leastBricks(wall));
    }
}
