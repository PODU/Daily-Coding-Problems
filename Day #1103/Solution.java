// Day 1103: Count right/down paths through a grid avoiding walls (1).
// DP: dp[i][j] = dp[i-1][j] + dp[i][j-1], 0 at walls. Time: O(N*M). Space: O(M).
public class Solution {
    static long countPaths(int[][] g){
        int n = g.length, m = g[0].length;
        long[] dp = new long[m];
        for (int i = 0; i < n; i++)
            for (int j = 0; j < m; j++){
                if (g[i][j] == 1) { dp[j] = 0; continue; }
                if (i == 0 && j == 0) dp[j] = 1;
                else dp[j] = (j > 0 ? dp[j-1] : 0) + dp[j];
            }
        return dp[m-1];
    }
    public static void main(String[] args){
        int[][] g = {{0,0,1},{0,0,1},{1,0,0}};
        System.out.println(countPaths(g)); // 2
    }
}
