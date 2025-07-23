// Paint House: DP tracking min cost per color using min1/min2 trick.
// Time O(N*K), Space O(1) extra.
public class Solution {
    static int minCost(int[][] cost) {
        if (cost.length == 0) return 0;
        int K = cost[0].length;
        int[] prev = cost[0].clone();
        for (int i = 1; i < cost.length; i++) {
            int min1 = -1, min2 = -1;
            for (int k = 0; k < K; k++) {
                if (min1 == -1 || prev[k] < prev[min1]) { min2 = min1; min1 = k; }
                else if (min2 == -1 || prev[k] < prev[min2]) { min2 = k; }
            }
            int[] cur = new int[K];
            for (int k = 0; k < K; k++) {
                int best = (k == min1) ? prev[min2] : prev[min1];
                cur[k] = cost[i][k] + best;
            }
            prev = cur;
        }
        int ans = Integer.MAX_VALUE;
        for (int v : prev) ans = Math.min(ans, v);
        return ans;
    }

    public static void main(String[] args) {
        int[][] cost = {{1,5,3},{2,9,4}};
        System.out.println("Minimum cost = " + minCost(cost));
    }
}
