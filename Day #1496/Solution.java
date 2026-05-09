// Day 1496: Min cost to paint N houses with K colors, no two adjacent same color.
// Approach: DP tracking previous row's min & 2nd-min (+min index). Time O(N*K), Space O(1).
public class Solution {
    static int minCost(int[][] costs) {
        if (costs.length == 0) return 0;
        long prevMin1 = 0, prevMin2 = 0;
        int prevIdx = -1;
        for (int[] row : costs) {
            long curMin1 = Long.MAX_VALUE, curMin2 = Long.MAX_VALUE;
            int curIdx = -1;
            for (int k = 0; k < row.length; k++) {
                long add = (k == prevIdx) ? prevMin2 : prevMin1;
                long c = row[k] + add;
                if (c < curMin1) { curMin2 = curMin1; curMin1 = c; curIdx = k; }
                else if (c < curMin2) { curMin2 = c; }
            }
            prevMin1 = curMin1; prevMin2 = curMin2; prevIdx = curIdx;
        }
        return (int) prevMin1;
    }

    public static void main(String[] args) {
        int[][] costs = {{1, 5, 3}, {2, 9, 4}};
        System.out.println(minCost(costs)); // expected 5
    }
}
