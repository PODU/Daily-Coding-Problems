// Day 717: Min cost to paint N houses, K colors, no two adjacent same color.
// DP over rows tracking best & second-best previous costs. Time O(N*K), space O(1).
import java.util.*;

public class Solution {
    static int minCost(int[][] costs) {
        if (costs.length == 0) return 0;
        int K = costs[0].length;
        int[] prev = costs[0].clone();
        for (int i = 1; i < costs.length; i++) {
            int m1 = Integer.MAX_VALUE, m2 = Integer.MAX_VALUE, idx = -1;
            for (int k = 0; k < K; k++) {
                if (prev[k] < m1) { m2 = m1; m1 = prev[k]; idx = k; }
                else if (prev[k] < m2) m2 = prev[k];
            }
            int[] cur = new int[K];
            for (int k = 0; k < K; k++)
                cur[k] = costs[i][k] + (k == idx ? m2 : m1);
            prev = cur;
        }
        int ans = Integer.MAX_VALUE;
        for (int v : prev) ans = Math.min(ans, v);
        return ans;
    }

    public static void main(String[] args) {
        int[][] costs = {{1, 5, 3}, {2, 9, 4}};
        System.out.println(minCost(costs));
    }
}
