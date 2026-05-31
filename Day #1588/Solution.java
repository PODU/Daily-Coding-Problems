// Count right/down paths in a grid with walls. DP: dp[i][j] = paths to cell (0 if wall).
// Time: O(N*M); Space: O(N*M).
public class Solution {
    static long countPaths(int[][] grid){
        int n=grid.length, m=grid[0].length;
        if(grid[0][0]==1||grid[n-1][m-1]==1) return 0;
        long[][] dp=new long[n][m];
        for(int i=0;i<n;i++){
            for(int j=0;j<m;j++){
                if(grid[i][j]==1){dp[i][j]=0;continue;}
                if(i==0&&j==0){dp[i][j]=1;continue;}
                long up=(i>0)?dp[i-1][j]:0;
                long left=(j>0)?dp[i][j-1]:0;
                dp[i][j]=up+left;
            }
        }
        return dp[n-1][m-1];
    }
    public static void main(String[] args){
        int[][] grid={{0,0,1},{0,0,1},{1,0,0}};
        System.out.println(countPaths(grid));
    }
}
