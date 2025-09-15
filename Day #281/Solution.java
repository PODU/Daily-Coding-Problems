// Day 281: Fewest bricks cut by a vertical line. Count prefix-sum edge positions;
// answer = rows - max edge frequency. Time O(total bricks), Space O(distinct edges).
import java.util.HashMap;
import java.util.Map;

public class Solution {
    static int fewestCuts(int[][] wall) {
        Map<Long, Integer> edge = new HashMap<>();
        int best = 0;
        for (int[] row : wall) {
            long sum = 0;
            for (int i = 0; i + 1 < row.length; i++) {
                sum += row[i];
                int c = edge.merge(sum, 1, Integer::sum);
                best = Math.max(best, c);
            }
        }
        return wall.length - best;
    }

    public static void main(String[] args) {
        int[][] wall = {
            {3, 5, 1, 1}, {2, 3, 3, 2}, {5, 5},
            {4, 4, 2}, {1, 3, 3, 3}, {1, 1, 6, 1, 1}
        };
        System.out.println(fewestCuts(wall)); // 2
    }
}
