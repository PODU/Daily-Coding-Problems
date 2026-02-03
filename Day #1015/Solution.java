// Largest path value in a directed graph: DFS topological memo + color cycle detection.
// dp[u][c] = max count of letter c on a path ending at u. Cycle -> null. O((n+m)*26) time, O(n*26) space.
import java.util.*;

public class Solution {
    static List<List<Integer>> adj;
    static int[][] dp;
    static int[] state; // 0 unvisited, 1 in-stack, 2 done
    static String colors;

    static boolean dfs(int u){
        state[u]=1;
        for(int v: adj.get(u)){
            if(state[v]==1) return false;       // back edge -> cycle
            if(state[v]==0){ if(!dfs(v)) return false; }
        }
        for(int v: adj.get(u))
            for(int c=0;c<26;c++) dp[u][c]=Math.max(dp[u][c],dp[v][c]);
        dp[u][colors.charAt(u)-'A']++;
        state[u]=2;
        return true;
    }

    static Integer largestPathValue(String cols, int[][] edges){
        int n=cols.length();
        colors=cols;
        adj=new ArrayList<>();
        for(int i=0;i<n;i++) adj.add(new ArrayList<>());
        for(int[] e: edges) adj.get(e[0]).add(e[1]);
        dp=new int[n][26];
        state=new int[n];
        for(int i=0;i<n;i++)
            if(state[i]==0 && !dfs(i)) return null; // cycle
        int ans=0;
        for(int i=0;i<n;i++) for(int c=0;c<26;c++) ans=Math.max(ans,dp[i][c]);
        return ans;
    }

    public static void main(String[] args){
        Integer r1=largestPathValue("ABACA", new int[][]{{0,1},{0,2},{2,3},{3,4}});
        System.out.println(r1==null?"null":r1);
        Integer r2=largestPathValue("A", new int[][]{{0,0}});
        System.out.println(r2==null?"null":r2);
    }
}
