// Paint houses: DP tracking two smallest costs of previous row -> O(N*K) time, O(1) extra space.
// For each house we only need the min and second-min of the previous row to avoid same color.
public class Solution {
    static int minCost(int[][] costs) {
        if (costs.length == 0) return 0;
        int prevMin = 0, prevSecond = 0, prevIdx = -1;
        for (int[] row : costs) {
            int curMin = Integer.MAX_VALUE, curSecond = Integer.MAX_VALUE, curIdx = -1;
            for (int c = 0; c < row.length; c++) {
                int cost = row[c] + (c == prevIdx ? prevSecond : prevMin);
                if (cost < curMin) {
                    curSecond = curMin;
                    curMin = cost;
                    curIdx = c;
                } else if (cost < curSecond) {
                    curSecond = cost;
                }
            }
            prevMin = curMin;
            prevSecond = curSecond;
            prevIdx = curIdx;
        }
        return prevMin;
    }

    public static void main(String[] args) {
        int[][] costs = {{1, 5, 3}, {2, 9, 4}};
        System.out.println(minCost(costs));
    }
}
